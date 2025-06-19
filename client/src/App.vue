<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 响应式数据
const agentUrl = ref("localhost:8080");
const isConnected = ref(false);
const connectionStatus = ref("未连接");
const metrics = ref(null);
const errorMessage = ref("");

// 轮询间隔
let pollInterval = null;

// 连接到Agent
async function connectToAgent() {
  try {
    errorMessage.value = "";
    connectionStatus.value = "连接中...";
    
    const result = await invoke("connect_to_agent", { agentUrl: agentUrl.value });
    
    isConnected.value = true;
    connectionStatus.value = "已连接";
    
    // 开始轮询获取指标数据
    startPolling();
    
  } catch (error) {
    console.error("连接失败:", error);
    errorMessage.value = error.toString();
    connectionStatus.value = "连接失败";
    isConnected.value = false;
  }
}

// 开始轮询获取指标数据
function startPolling() {
  if (pollInterval) {
    clearInterval(pollInterval);
  }
  
  pollInterval = setInterval(async () => {
    try {
      const latestMetrics = await invoke("get_latest_metrics");
      if (latestMetrics) {
        metrics.value = latestMetrics;
      }
    } catch (error) {
      console.error("获取指标失败:", error);
    }
  }, 1000); // 每秒获取一次指标
}

// 停止轮询
function stopPolling() {
  if (pollInterval) {
    clearInterval(pollInterval);
    pollInterval = null;
  }
}

// 断开连接
function disconnect() {
  stopPolling();
  isConnected.value = false;
  connectionStatus.value = "未连接";
  metrics.value = null;
}

// 格式化字节数
function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// 格式化时间戳
function formatTimestamp(timestamp) {
  return new Date(timestamp * 1000).toLocaleTimeString();
}

// 获取进度条颜色
function getProgressColor(percentage) {
  if (percentage < 50) return '#4CAF50'; // 绿色
  if (percentage < 80) return '#FF9800'; // 橙色
  return '#F44336'; // 红色
}

// 组件挂载时的处理
onMounted(() => {
  // 可以在这里做一些初始化
});

// 组件卸载时的清理
onUnmounted(() => {
  stopPolling();
});
</script>

<template>
  <main class="container">
    <h1>Light Guide - 系统监控</h1>

    <!-- 连接控制面板 -->
    <div class="connection-panel">
      <div class="connection-form">
        <input 
          v-model="agentUrl" 
          placeholder="Agent地址 (例如: localhost:8080)"
          :disabled="isConnected"
          class="agent-input"
        />
        <button 
          @click="connectToAgent" 
          :disabled="isConnected"
          class="connect-btn"
        >
          连接
        </button>
        <button 
          @click="disconnect" 
          :disabled="!isConnected"
          class="disconnect-btn"
        >
          断开
        </button>
      </div>
      
      <div class="status-info">
        <span class="status-label">状态:</span>
        <span :class="['status-value', isConnected ? 'connected' : 'disconnected']">
          {{ connectionStatus }}
        </span>
      </div>
      
      <div v-if="errorMessage" class="error-message">
        {{ errorMessage }}
      </div>
    </div>

    <!-- 系统指标展示 -->
    <div v-if="isConnected && metrics" class="metrics-dashboard">
      <div class="metrics-header">
        <h2>系统指标</h2>
        <div class="last-update">
          最后更新: {{ formatTimestamp(metrics.timestamp) }}
        </div>
      </div>

      <div class="metrics-grid">
        <!-- CPU使用率 -->
        <div class="metric-card">
          <div class="metric-title">CPU 使用率</div>
          <div class="metric-value">{{ metrics.cpu_usage.toFixed(1) }}%</div>
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ 
                width: metrics.cpu_usage + '%', 
                backgroundColor: getProgressColor(metrics.cpu_usage) 
              }"
            ></div>
          </div>
        </div>

        <!-- 内存使用率 -->
        <div class="metric-card">
          <div class="metric-title">内存使用率</div>
          <div class="metric-value">{{ metrics.memory_usage.toFixed(1) }}%</div>
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ 
                width: metrics.memory_usage + '%', 
                backgroundColor: getProgressColor(metrics.memory_usage) 
              }"
            ></div>
          </div>
        </div>

        <!-- 磁盘使用率 -->
        <div class="metric-card">
          <div class="metric-title">磁盘使用率</div>
          <div class="metric-value">{{ metrics.disk_usage.toFixed(1) }}%</div>
          <div class="progress-bar">
            <div 
              class="progress-fill" 
              :style="{ 
                width: metrics.disk_usage + '%', 
                backgroundColor: getProgressColor(metrics.disk_usage) 
              }"
            ></div>
          </div>
        </div>

        <!-- 网络流量 -->
        <div class="metric-card network-card">
          <div class="metric-title">网络流量</div>
          <div class="network-stats">
            <div class="network-stat">
              <span class="network-label">↓ 接收:</span>
              <span class="network-value">{{ formatBytes(metrics.network_in) }}</span>
            </div>
            <div class="network-stat">
              <span class="network-label">↑ 发送:</span>
              <span class="network-value">{{ formatBytes(metrics.network_out) }}</span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 未连接时的提示 -->
    <div v-else-if="!isConnected" class="no-connection">
      <div class="no-connection-icon">📊</div>
      <h3>请连接到Agent开始监控</h3>
      <p>输入Agent地址并点击连接按钮</p>
    </div>

    <!-- 已连接但没有数据的提示 -->
    <div v-else class="waiting-data">
      <div class="loading-spinner"></div>
      <h3>等待数据中...</h3>
      <p>已连接到Agent，正在获取系统指标</p>
    </div>
  </main>
</template>

<style scoped>
/* 连接面板样式 */
.connection-panel {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  margin-bottom: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e1e1e1;
}

.connection-form {
  display: flex;
  gap: 12px;
  margin-bottom: 16px;
  align-items: center;
  flex-wrap: wrap;
}

.agent-input {
  flex: 1;
  min-width: 250px;
  padding: 12px 16px;
  border: 2px solid #e1e1e1;
  border-radius: 8px;
  font-size: 14px;
  transition: border-color 0.3s;
}

.agent-input:focus {
  outline: none;
  border-color: #4CAF50;
}

.agent-input:disabled {
  background-color: #f5f5f5;
  color: #999;
}

.connect-btn, .disconnect-btn {
  padding: 12px 24px;
  border: none;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s;
}

.connect-btn {
  background: #4CAF50;
  color: white;
}

.connect-btn:hover:not(:disabled) {
  background: #45a049;
  transform: translateY(-1px);
}

.connect-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.disconnect-btn {
  background: #f44336;
  color: white;
}

.disconnect-btn:hover:not(:disabled) {
  background: #da190b;
  transform: translateY(-1px);
}

.disconnect-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
}

.status-info {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.status-label {
  font-weight: 600;
  color: #666;
}

.status-value {
  font-weight: 600;
  padding: 4px 12px;
  border-radius: 20px;
  font-size: 13px;
}

.status-value.connected {
  background: #e8f5e8;
  color: #2e7d2e;
}

.status-value.disconnected {
  background: #ffeaea;
  color: #c62828;
}

.error-message {
  margin-top: 12px;
  padding: 12px;
  background: #ffebee;
  color: #c62828;
  border-radius: 6px;
  font-size: 14px;
  border-left: 4px solid #f44336;
}

/* 指标仪表板样式 */
.metrics-dashboard {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
  border: 1px solid #e1e1e1;
}

.metrics-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid #f0f0f0;
}

.metrics-header h2 {
  margin: 0;
  color: #333;
  font-size: 24px;
}

.last-update {
  color: #666;
  font-size: 14px;
  background: #f9f9f9;
  padding: 6px 12px;
  border-radius: 20px;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 20px;
}

.metric-card {
  background: #fafafa;
  border-radius: 12px;
  padding: 20px;
  border: 2px solid #f0f0f0;
  transition: all 0.3s;
}

.metric-card:hover {
  border-color: #4CAF50;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.metric-title {
  font-size: 14px;
  font-weight: 600;
  color: #666;
  margin-bottom: 8px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.metric-value {
  font-size: 32px;
  font-weight: 700;
  color: #333;
  margin-bottom: 16px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: #e0e0e0;
  border-radius: 4px;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 4px;
  transition: all 0.5s ease;
}

.network-card {
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.network-stats {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.network-stat {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 0;
}

.network-label {
  font-weight: 600;
  color: #666;
  font-size: 14px;
}

.network-value {
  font-weight: 700;
  color: #333;
  font-size: 16px;
  background: rgba(255, 255, 255, 0.7);
  padding: 4px 8px;
  border-radius: 4px;
}

/* 无连接状态样式 */
.no-connection {
  text-align: center;
  padding: 60px 20px;
  color: #666;
}

.no-connection-icon {
  font-size: 64px;
  margin-bottom: 20px;
}

.no-connection h3 {
  color: #333;
  margin-bottom: 8px;
}

.no-connection p {
  color: #666;
  font-size: 14px;
}

/* 等待数据状态样式 */
.waiting-data {
  text-align: center;
  padding: 60px 20px;
  color: #666;
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f0f0f0;
  border-top: 4px solid #4CAF50;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

.waiting-data h3 {
  color: #333;
  margin-bottom: 8px;
}

.waiting-data p {
  color: #666;
  font-size: 14px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

/* 响应式设计 */
@media (max-width: 768px) {
  .connection-form {
    flex-direction: column;
    align-items: stretch;
  }
  
  .agent-input {
    min-width: auto;
  }
  
  .metrics-grid {
    grid-template-columns: 1fr;
  }
  
  .metrics-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 12px;
  }
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
