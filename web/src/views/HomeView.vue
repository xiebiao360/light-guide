<script setup>
import { onMounted } from 'vue'
import { useTabStore } from '@/stores/tabs'

const tabStore = useTabStore()

onMounted(() => {
  tabStore.initializeHomePage()
})
</script>

<template>
  <div class="home-view">
    <div class="welcome-section">
      <h1>欢迎使用 Light Guide 📚</h1>
      <p class="welcome-text">这是一个现代化的管理平台，提供丰富的功能和优秀的用户体验。</p>
    </div>

    <div class="features-grid">
      <div class="feature-card">
        <div class="feature-icon">📊</div>
        <h3>数据分析</h3>
        <p>强大的数据分析和可视化功能</p>
      </div>

      <div class="feature-card">
        <div class="feature-icon">👥</div>
        <h3>用户管理</h3>
        <p>完整的用户权限和角色管理</p>
      </div>

      <div class="feature-card">
        <div class="feature-icon">⚙️</div>
        <h3>系统配置</h3>
        <p>灵活的系统设置和个性化配置</p>
      </div>

      <div class="feature-card">
        <div class="feature-icon">🔒</div>
        <h3>安全保障</h3>
        <p>企业级安全策略和数据保护</p>
      </div>
    </div>

    <div class="stats-section">
      <div class="stat-item">
        <div class="stat-number">1,234</div>
        <div class="stat-label">总用户数</div>
      </div>
      <div class="stat-item">
        <div class="stat-number">89</div>
        <div class="stat-label">活跃项目</div>
      </div>
      <div class="stat-item">
        <div class="stat-number">567</div>
        <div class="stat-label">今日访问</div>
      </div>
      <div class="stat-item">
        <div class="stat-number">99.9%</div>
        <div class="stat-label">系统可用性</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-view {
  width: 100%;
  padding: 0 2rem;
}

.welcome-section {
  text-align: center;
  margin-bottom: 3rem;
  padding: 2rem;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.welcome-section h1 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
  font-weight: 700;
}

.welcome-text {
  font-size: 1.2rem;
  opacity: 0.9;
  max-width: 600px;
  margin: 0 auto;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin-bottom: 3rem;
  max-width: none;
}

.feature-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  text-align: center;
  transition:
    transform 0.3s ease,
    box-shadow 0.3s ease;
  border: 1px solid #f0f0f0;
}

.feature-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.12);
}

.feature-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.feature-card h3 {
  color: #333;
  margin-bottom: 1rem;
  font-size: 1.3rem;
  font-weight: 600;
}

.feature-card p {
  color: #666;
  line-height: 1.6;
}

.stats-section {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 2rem;
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  max-width: none;
}

.stat-item {
  text-align: center;
  padding: 1rem;
}

.stat-number {
  font-size: 2.5rem;
  font-weight: 700;
  color: #667eea;
  margin-bottom: 0.5rem;
}

.stat-label {
  color: #666;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
</style>
