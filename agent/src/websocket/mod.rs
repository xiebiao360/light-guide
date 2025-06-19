// WebSocket 通信模块
// 负责与客户端的 WebSocket 连接和消息处理

use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Message, Result},
};
use tokio::net::{TcpListener, TcpStream};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;
use tracing::{info, warn, error};

use crate::system::{SystemMetrics, SystemInfo};

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

pub struct WebSocketServer {
    addr: String,
    message_sender: broadcast::Sender<AgentMessage>,
}

impl WebSocketServer {
    pub fn new(addr: String) -> Self {
        let (message_sender, _) = broadcast::channel(1000);
        
        Self {
            addr,
            message_sender,
        }
    }
    
    /// 启动WebSocket服务器
    pub async fn start(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await
            .map_err(|e| Error::Io(e))?;
        
        info!("WebSocket服务器启动在: {}", self.addr);
        
        while let Ok((stream, addr)) = listener.accept().await {
            info!("新的客户端连接: {}", addr);
            
            let message_sender = self.message_sender.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_connection(stream, message_sender).await {
                    error!("处理连接错误: {}", e);
                }
            });
        }
        
        Ok(())
    }
    
    /// 发送消息给所有连接的客户端
    pub fn broadcast_message(&self, message: AgentMessage) -> Result<()> {
        match self.message_sender.send(message) {
            Ok(_) => Ok(()),
            Err(_) => {
                warn!("没有活跃的客户端连接");
                Ok(())
            }
        }
    }
    
    /// 获取消息发送器的克隆，用于其他模块发送消息
    pub fn get_message_sender(&self) -> broadcast::Sender<AgentMessage> {
        self.message_sender.clone()
    }
}

async fn handle_connection(
    stream: TcpStream,
    message_sender: broadcast::Sender<AgentMessage>,
) -> Result<()> {
    let websocket = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = websocket.split();
    
    let mut message_receiver = message_sender.subscribe();
    
    // 处理从Agent内部发送到客户端的消息
    let send_task = tokio::spawn(async move {
        while let Ok(agent_message) = message_receiver.recv().await {
            let json_message = serde_json::to_string(&agent_message)
                .unwrap_or_else(|_| "{}".to_string());
            
            if let Err(e) = ws_sender.send(Message::Text(json_message)).await {
                error!("发送消息失败: {}", e);
                break;
            }
        }
    });
    
    // 处理从客户端收到的消息
    let receive_task = tokio::spawn(async move {
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    if let Ok(client_message) = serde_json::from_str::<ClientMessage>(&text) {
                        handle_client_message(client_message, &message_sender).await;
                    } else {
                        warn!("无法解析客户端消息: {}", text);
                    }
                },
                Ok(Message::Close(_)) => {
                    info!("客户端断开连接");
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
    
    // 等待任一任务完成
    tokio::select! {
        _ = send_task => {},
        _ = receive_task => {},
    }
    
    info!("客户端连接已关闭");
    Ok(())
}

async fn handle_client_message(
    message: ClientMessage,
    _message_sender: &broadcast::Sender<AgentMessage>,
) {
    match message {
        ClientMessage::RequestSystemInfo => {
            info!("客户端请求系统信息");
            // TODO: 发送系统信息
        },
        ClientMessage::StartMonitoring { interval_secs } => {
            info!("客户端请求开始监控，间隔: {}秒", interval_secs);
            // TODO: 启动监控
        },
        ClientMessage::StopMonitoring => {
            info!("客户端请求停止监控");
            // TODO: 停止监控
        },
        ClientMessage::Heartbeat => {
            // 心跳消息，不需要特殊处理
        },
    }
}