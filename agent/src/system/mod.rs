// 系统监控模块
// 负责收集系统指标：CPU、内存、磁盘、网络等

use sysinfo::System;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{interval, Duration};
use tracing::info;

// 导入共享类型
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub disk_usage: f64,
    pub network_in: u64,
    pub network_out: u64,
    pub timestamp: u64,
}

pub struct SystemMonitor {
    system: System,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        
        Self { system }
    }
    
    /// 收集当前系统指标
    pub fn collect_metrics(&mut self) -> SystemMetrics {
        // 刷新系统信息
        self.system.refresh_all();
        
        // 获取当前时间戳
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        // 计算CPU使用率（所有核心的平均值）
        let cpu_usage = self.system.cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .sum::<f32>() / self.system.cpus().len() as f32;
        
        // 计算内存使用率
        let total_memory = self.system.total_memory();
        let used_memory = self.system.used_memory();
        let memory_usage = if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        };
        
        // 计算磁盘使用率（使用简化方式）
        let disk_usage = 0.0; // 暂时设为0，后续可以使用其他方式获取
        
        // 计算网络流量（使用简化方式）
        let (network_in, network_out) = (0u64, 0u64); // 暂时设为0，后续可以使用其他方式获取
        
        SystemMetrics {
            cpu_usage: cpu_usage as f64,
            memory_usage,
            disk_usage,
            network_in,
            network_out,
            timestamp,
        }
    }
    
    /// 启动定期指标收集
    pub async fn start_monitoring<F>(&mut self, interval_secs: u64, callback: F)
    where
        F: Fn(SystemMetrics) + Send + 'static,
    {
        info!("启动系统监控，采集间隔: {}秒", interval_secs);
        
        let mut interval = interval(Duration::from_secs(interval_secs));
        
        loop {
            interval.tick().await;
            
            let metrics = self.collect_metrics();
            info!(
                "系统指标 - CPU: {:.1}%, 内存: {:.1}%, 磁盘: {:.1}%, 网络进: {}B, 网络出: {}B",
                metrics.cpu_usage,
                metrics.memory_usage,
                metrics.disk_usage,
                metrics.network_in,
                metrics.network_out
            );
            
            callback(metrics);
        }
    }
    
    /// 获取系统基本信息
    pub fn get_system_info(&mut self) -> SystemInfo {
        self.system.refresh_all();
        
        SystemInfo {
            hostname: System::host_name().unwrap_or_else(|| "unknown".to_string()),
            os_name: System::name().unwrap_or_else(|| "unknown".to_string()),
            os_version: System::os_version().unwrap_or_else(|| "unknown".to_string()),
            kernel_version: System::kernel_version().unwrap_or_else(|| "unknown".to_string()),
            total_memory: self.system.total_memory(),
            cpu_count: self.system.cpus().len() as u32,
            uptime: System::uptime(),
        }
    }
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