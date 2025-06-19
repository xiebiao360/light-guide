// 简单的WebSocket客户端测试脚本
const WebSocket = require('ws');

const ws = new WebSocket('ws://localhost:8080');

ws.on('open', function open() {
  console.log('✅ WebSocket连接已建立');
  
  // 发送开始监控命令
  const startMonitoring = {
    StartMonitoring: { interval_secs: 5 }
  };
  
  ws.send(JSON.stringify(startMonitoring));
  console.log('📤 已发送开始监控命令');
});

ws.on('message', function message(data) {
  console.log('📥 收到数据:', data.toString());
  
  try {
    const parsed = JSON.parse(data.toString());
    if (parsed.SystemMetrics) {
      const metrics = parsed.SystemMetrics;
      console.log(`🖥️  系统指标 - CPU: ${metrics.cpu_usage.toFixed(1)}%, 内存: ${metrics.memory_usage.toFixed(1)}%, 磁盘: ${metrics.disk_usage.toFixed(1)}%`);
    }
  } catch (e) {
    console.log('⚠️  无法解析消息:', e.message);
  }
});

ws.on('error', function error(err) {
  console.error('❌ WebSocket错误:', err.message);
});

ws.on('close', function close() {
  console.log('🔌 WebSocket连接已关闭');
});

// 10秒后关闭连接
setTimeout(() => {
  console.log('⏰ 测试完成，关闭连接');
  ws.close();
}, 10000);