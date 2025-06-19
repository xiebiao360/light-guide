use serde::{Deserialize, Serialize};
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use tracing::{info, warn, error};

// 系统指标数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_in: u64,
    pub network_out: u64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub total_memory: u64,
    pub cpu_count: u32,
    pub uptime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentMessage {
    SystemMetrics(SystemMetrics),
    SystemInfo(SystemInfo),
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    RequestSystemInfo,
    StartMonitoring { interval_secs: u64 },
    StopMonitoring,
    Heartbeat,
}

// 全局WebSocket连接管理器
pub struct WebSocketClient {
    sender: Option<broadcast::Sender<SystemMetrics>>,
}

impl WebSocketClient {
    pub fn new() -> Self {
        Self { sender: None }
    }
}

// 全局WebSocket客户端实例
static WS_CLIENT: once_cell::sync::Lazy<Arc<Mutex<WebSocketClient>>> = 
    once_cell::sync::Lazy::new(|| Arc::new(Mutex::new(WebSocketClient::new())));

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn connect_to_agent(agent_url: String) -> Result<String, String> {
    info!("连接到Agent: {}", agent_url);
    
    let url = format!("ws://{}", agent_url);
    
    match connect_async(&url).await {
        Ok((ws_stream, _)) => {
            info!("成功连接到Agent: {}", agent_url);
            
            let (mut ws_sender, mut ws_receiver) = ws_stream.split();
            let (metrics_sender, _) = broadcast::channel::<SystemMetrics>(1000);
            
            // 更新全局WebSocket客户端
            {
                let mut client = WS_CLIENT.lock().await;
                client.sender = Some(metrics_sender.clone());
            }
            
            // 启动消息接收任务
            tokio::spawn(async move {
                while let Some(msg) = ws_receiver.next().await {
                    match msg {
                        Ok(Message::Text(text)) => {
                            if let Ok(agent_message) = serde_json::from_str::<AgentMessage>(&text) {
                                match agent_message {
                                    AgentMessage::SystemMetrics(metrics) => {
                                        let _ = metrics_sender.send(metrics);
                                    },
                                    AgentMessage::SystemInfo(info) => {
                                        info!("收到系统信息: {:?}", info);
                                    },
                                    AgentMessage::Heartbeat => {
                                        // 处理心跳
                                    }
                                }
                            }
                        },
                        Ok(Message::Close(_)) => {
                            warn!("Agent连接已关闭");
                            break;
                        },
                        Err(e) => {
                            error!("接收消息错误: {}", e);
                            break;
                        },
                        _ => {}
                    }
                }
            });
            
            // 发送开始监控命令
            let start_monitoring = ClientMessage::StartMonitoring { interval_secs: 5 };
            let message_text = serde_json::to_string(&start_monitoring).unwrap();
            
            tokio::spawn(async move {
                if let Err(e) = ws_sender.send(Message::Text(message_text)).await {
                    error!("发送监控启动命令失败: {}", e);
                }
            });
            
            Ok("连接成功".to_string())
        },
        Err(e) => {
            error!("连接Agent失败: {}", e);
            Err(format!("连接失败: {}", e))
        }
    }
}

#[tauri::command]
async fn get_latest_metrics() -> Result<Option<SystemMetrics>, String> {
    let client = WS_CLIENT.lock().await;
    
    if let Some(sender) = &client.sender {
        let mut receiver = sender.subscribe();
        
        // 等待最新的指标数据
        match tokio::time::timeout(std::time::Duration::from_millis(100), receiver.recv()).await {
            Ok(Ok(metrics)) => Ok(Some(metrics)),
            Ok(Err(_)) => Ok(None),
            Err(_) => Ok(None), // 超时
        }
    } else {
        Ok(None)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, connect_to_agent, get_latest_metrics])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
