use clap::{Arg, Command};
use tracing::{info, error};

mod websocket;
mod system;
mod apps;
mod logs;
mod packages;
mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化日志
    tracing_subscriber::init();

    let matches = Command::new("light-guide-agent")
        .version("0.1.0")
        .about("Light Guide Agent - 多环境运维代理")
        .arg(
            Arg::new("server")
                .short('s')
                .long("server")
                .value_name("URL")
                .help("客户端服务器地址")
                .required(true),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("监听端口")
                .default_value("8080"),
        )
        .get_matches();

    let server_url = matches.get_one::<String>("server").unwrap();
    let port = matches.get_one::<String>("port").unwrap();

    info!("启动 Light Guide Agent");
    info!("服务器地址: {}", server_url);
    info!("监听端口: {}", port);

    // TODO: 启动 WebSocket 连接和各个模块
    
    Ok(())
}