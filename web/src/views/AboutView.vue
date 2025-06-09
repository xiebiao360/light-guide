<template>
  <div class="about-view">
    <div class="page-header">
      <h1>ℹ️ 关于</h1>
      <p>了解 Light Guide 平台的详细信息</p>
    </div>

    <div class="about-content">
      <div class="info-card">
        <div class="app-logo">💡</div>
        <h2>Light Guide</h2>
        <p class="version">版本 1.0.0</p>
        <p class="description">
          Light Guide 是一个现代化的管理平台，采用 Vue 3 + Rust 技术栈构建，
          为用户提供高效、安全、易用的管理体验。
        </p>
      </div>

      <div class="tech-stack">
        <h3>技术栈</h3>
        <div class="tech-grid">
          <div class="tech-item">
            <div class="tech-icon">⚡</div>
            <div class="tech-info">
              <h4>Vue 3</h4>
              <p>前端框架</p>
            </div>
          </div>
          <div class="tech-item">
            <div class="tech-icon">🦀</div>
            <div class="tech-info">
              <h4>Rust</h4>
              <p>后端语言</p>
            </div>
          </div>
          <div class="tech-item">
            <div class="tech-icon">🔥</div>
            <div class="tech-info">
              <h4>Axum</h4>
              <p>Web 框架</p>
            </div>
          </div>
          <div class="tech-item">
            <div class="tech-icon">🗃️</div>
            <div class="tech-info">
              <h4>SQLite</h4>
              <p>数据库</p>
            </div>
          </div>
          <div class="tech-item">
            <div class="tech-icon">🎨</div>
            <div class="tech-info">
              <h4>Vite</h4>
              <p>构建工具</p>
            </div>
          </div>
          <div class="tech-item">
            <div class="tech-icon">🍍</div>
            <div class="tech-info">
              <h4>Pinia</h4>
              <p>状态管理</p>
            </div>
          </div>
        </div>
      </div>

      <div class="features-list">
        <h3>主要特性</h3>
        <ul>
          <li>🎯 现代化的用户界面设计</li>
          <li>📱 响应式布局，支持多设备</li>
          <li>🔒 企业级安全保障</li>
          <li>⚡ 高性能的数据处理</li>
          <li>🔧 灵活的系统配置</li>
          <li>📊 丰富的数据可视化</li>
          <li>👥 完善的用户权限管理</li>
          <li>🌍 国际化支持</li>
        </ul>
      </div>

      <div class="contact-info">
        <h3>联系信息</h3>
        <div class="contact-grid">
          <div class="contact-item">
            <span class="contact-label">开发团队：</span>
            <span class="contact-value">Light Guide Team</span>
          </div>
          <div class="contact-item">
            <span class="contact-label">邮箱：</span>
            <span class="contact-value">support@lightguide.com</span>
          </div>
          <div class="contact-item">
            <span class="contact-label">官网：</span>
            <span class="contact-value">https://lightguide.com</span>
          </div>
          <div class="contact-item">
            <span class="contact-label">Github：</span>
            <span class="contact-value">https://github.com/lightguide</span>
          </div>
        </div>
      </div>

      <div class="copyright">
        <p>&copy; 2024 Light Guide. All rights reserved.</p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.about-view {
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

.about-content {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.info-card {
  background: white;
  padding: 3rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  text-align: center;
}

.app-logo {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.info-card h2 {
  color: #333;
  margin-bottom: 0.5rem;
  font-size: 2.5rem;
  font-weight: 700;
}

.version {
  color: #1890ff;
  font-size: 1.1rem;
  font-weight: 600;
  margin-bottom: 1.5rem;
}

.description {
  color: #666;
  font-size: 1.1rem;
  line-height: 1.6;
  max-width: 600px;
  margin: 0 auto;
}

.tech-stack,
.features-list,
.contact-info {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
}

.tech-stack h3,
.features-list h3,
.contact-info h3 {
  color: #333;
  margin-bottom: 1.5rem;
  font-size: 1.4rem;
  font-weight: 600;
}

.tech-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
}

.tech-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-radius: 8px;
  transition: transform 0.3s ease;
}

.tech-item:hover {
  transform: translateY(-2px);
}

.tech-icon {
  font-size: 2rem;
}

.tech-info h4 {
  color: #333;
  margin: 0 0 0.25rem 0;
  font-size: 1.1rem;
}

.tech-info p {
  color: #666;
  margin: 0;
  font-size: 0.9rem;
}

.features-list ul {
  list-style: none;
  padding: 0;
  margin: 0;
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1rem;
}

.features-list li {
  color: #333;
  font-size: 1rem;
  padding: 0.5rem 0;
}

.contact-grid {
  display: grid;
  gap: 1rem;
}

.contact-item {
  display: flex;
  align-items: center;
  padding: 0.75rem 0;
  border-bottom: 1px solid #f0f0f0;
}

.contact-item:last-child {
  border-bottom: none;
}

.contact-label {
  color: #666;
  font-weight: 500;
  min-width: 120px;
}

.contact-value {
  color: #333;
  flex: 1;
}

.copyright {
  text-align: center;
  padding: 2rem;
  color: #999;
  font-size: 0.9rem;
}
</style>
