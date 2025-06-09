<template>
  <div class="dashboard-view">
    <div class="page-header">
      <h1>📊 仪表板</h1>
      <p>系统数据概览和关键指标监控</p>
    </div>

    <div class="metrics-grid">
      <div class="metric-card">
        <div class="metric-header">
          <h3>总收入</h3>
          <span class="trend up">↗ +12.5%</span>
        </div>
        <div class="metric-value">¥128,543</div>
        <div class="metric-period">本月</div>
      </div>

      <div class="metric-card">
        <div class="metric-header">
          <h3>新用户</h3>
          <span class="trend up">↗ +8.2%</span>
        </div>
        <div class="metric-value">1,234</div>
        <div class="metric-period">本周</div>
      </div>

      <div class="metric-card">
        <div class="metric-header">
          <h3>订单数量</h3>
          <span class="trend down">↘ -3.1%</span>
        </div>
        <div class="metric-value">892</div>
        <div class="metric-period">今日</div>
      </div>

      <div class="metric-card">
        <div class="metric-header">
          <h3>转化率</h3>
          <span class="trend up">↗ +2.4%</span>
        </div>
        <div class="metric-value">15.8%</div>
        <div class="metric-period">平均</div>
      </div>
    </div>

    <div class="charts-section">
      <div class="chart-card">
        <h3>销售趋势</h3>
        <div class="chart-placeholder">📈 图表区域 - 这里可以集成 Chart.js 或其他图表库</div>
      </div>

      <div class="chart-card">
        <h3>用户分布</h3>
        <div class="chart-placeholder">🥧 饼图区域 - 用户地理分布统计</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.dashboard-view {
  width: 100%;
  padding: 0 2rem;
}

.page-header {
  margin-bottom: 2rem;
}

.page-header h1 {
  color: #333;
  margin-bottom: 0.5rem;
  font-size: 2rem;
}

.page-header p {
  color: #666;
  font-size: 1.1rem;
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
  max-width: none;
}

.metric-card {
  background: white;
  padding: 1.5rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  border: 1px solid #f0f0f0;
}

.metric-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.metric-header h3 {
  color: #666;
  font-size: 0.9rem;
  font-weight: 500;
  margin: 0;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.trend {
  font-size: 0.8rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-weight: 600;
}

.trend.up {
  background: #f6ffed;
  color: #52c41a;
}

.trend.down {
  background: #fff2f0;
  color: #ff4d4f;
}

.metric-value {
  font-size: 2.2rem;
  font-weight: 700;
  color: #333;
  margin-bottom: 0.5rem;
}

.metric-period {
  color: #999;
  font-size: 0.8rem;
}

.charts-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(450px, 1fr));
  gap: 2rem;
  max-width: none;
}

.chart-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  border: 1px solid #f0f0f0;
}

.chart-card h3 {
  color: #333;
  margin-bottom: 1.5rem;
  font-size: 1.2rem;
}

.chart-placeholder {
  height: 250px;
  background: #f8f9fa;
  border: 2px dashed #dee2e6;
  border-radius: 8px;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #6c757d;
  font-size: 1.1rem;
  text-align: center;
}
</style>
