use clap::{Arg, Command};
use tracing::info;
use std::sync::Arc;

mod websocket;
mod system;
mod apps;
mod logs;
mod packages;
mod config;

use websocket::{WebSocketServer, AgentMessage};
use system::SystemMonitor;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();

    let matches = Command::new("light-guide-agent")
        .version("0.1.0")
        .about("Light Guide Agent - 多环境运维代理")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("监听端口")
                .default_value("8080"),
        )
        .get_matches();

    let port = matches.get_one::<String>("port").unwrap();
    let addr = format!("0.0.0.0:{}", port);

    info!("启动 Light Guide Agent");
    info!("监听地址: {}", addr);

    // 创建WebSocket服务器
    let ws_server = Arc::new(WebSocketServer::new(addr.clone()));
    let message_sender = ws_server.get_message_sender();
    
    // 创建系统监控器
    let mut system_monitor = SystemMonitor::new();
    
    // 启动系统监控任务
    let monitor_sender = message_sender.clone();
    tokio::spawn(async move {
        system_monitor.start_monitoring(5, move |metrics| {
            let _ = monitor_sender.send(AgentMessage::SystemMetrics(metrics));
        }).await;
    });

    // 启动WebSocket服务器
    info!("启动WebSocket服务器...");
    ws_server.start().await?;
    
    Ok(())
}