<template>
  <div class="settings-view">
    <div class="page-header">
      <h1>⚙️ 系统设置</h1>
      <p>配置系统参数和个人偏好设置</p>
    </div>

    <div class="settings-content">
      <div class="settings-menu">
        <div
          v-for="section in sections"
          :key="section.id"
          :class="['menu-item', { active: activeSection === section.id }]"
          @click="activeSection = section.id"
        >
          <span class="menu-icon">{{ section.icon }}</span>
          <span class="menu-text">{{ section.title }}</span>
        </div>
      </div>

      <div class="settings-panel">
        <div v-if="activeSection === 'general'" class="setting-section">
          <h2>常规设置</h2>
          <div class="setting-group">
            <div class="setting-item">
              <label>系统名称</label>
              <input type="text" v-model="settings.systemName" class="setting-input" />
            </div>
            <div class="setting-item">
              <label>系统描述</label>
              <textarea v-model="settings.systemDescription" class="setting-textarea"></textarea>
            </div>
            <div class="setting-item">
              <label>语言设置</label>
              <select v-model="settings.language" class="setting-select">
                <option value="zh-CN">简体中文</option>
                <option value="en-US">English</option>
                <option value="ja-JP">日本語</option>
              </select>
            </div>
          </div>
        </div>

        <div v-if="activeSection === 'security'" class="setting-section">
          <h2>安全设置</h2>
          <div class="setting-group">
            <div class="setting-item">
              <label>密码强度要求</label>
              <div class="checkbox-group">
                <label class="checkbox-item">
                  <input type="checkbox" v-model="settings.security.requireUppercase" />
                  <span>包含大写字母</span>
                </label>
                <label class="checkbox-item">
                  <input type="checkbox" v-model="settings.security.requireNumbers" />
                  <span>包含数字</span>
                </label>
                <label class="checkbox-item">
                  <input type="checkbox" v-model="settings.security.requireSymbols" />
                  <span>包含特殊字符</span>
                </label>
              </div>
            </div>
            <div class="setting-item">
              <label>会话超时时间（分钟）</label>
              <input
                type="number"
                v-model="settings.security.sessionTimeout"
                class="setting-input"
              />
            </div>
          </div>
        </div>

        <div v-if="activeSection === 'notifications'" class="setting-section">
          <h2>通知设置</h2>
          <div class="setting-group">
            <div class="setting-item">
              <label class="switch-item">
                <span>邮件通知</span>
                <input type="checkbox" v-model="settings.notifications.email" class="switch" />
              </label>
            </div>
            <div class="setting-item">
              <label class="switch-item">
                <span>短信通知</span>
                <input type="checkbox" v-model="settings.notifications.sms" class="switch" />
              </label>
            </div>
            <div class="setting-item">
              <label class="switch-item">
                <span>浏览器推送</span>
                <input type="checkbox" v-model="settings.notifications.push" class="switch" />
              </label>
            </div>
          </div>
        </div>

        <div class="setting-actions">
          <button class="btn-primary" @click="saveSettings">保存设置</button>
          <button class="btn-secondary" @click="resetSettings">重置</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const activeSection = ref('general')

const sections = [
  { id: 'general', title: '常规', icon: '🔧' },
  { id: 'security', title: '安全', icon: '🔒' },
  { id: 'notifications', title: '通知', icon: '🔔' },
]

const settings = ref({
  systemName: 'Light Guide',
  systemDescription: '现代化的管理平台',
  language: 'zh-CN',
  security: {
    requireUppercase: true,
    requireNumbers: true,
    requireSymbols: false,
    sessionTimeout: 30,
  },
  notifications: {
    email: true,
    sms: false,
    push: true,
  },
})

const saveSettings = () => {
  alert('设置已保存')
}

const resetSettings = () => {
  if (confirm('确定要重置所有设置吗？')) {
    location.reload()
  }
}
</script>

<style scoped>
.settings-view {
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

.settings-content {
  display: flex;
  gap: 2rem;
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

.settings-menu {
  width: 250px;
  background: #fafafa;
  padding: 1.5rem 0;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 1rem 1.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  border-right: 3px solid transparent;
}

.menu-item:hover {
  background: #f0f0f0;
}

.menu-item.active {
  background: white;
  border-right-color: #1890ff;
  color: #1890ff;
}

.menu-icon {
  margin-right: 0.75rem;
  font-size: 1.1rem;
}

.menu-text {
  font-weight: 500;
}

.settings-panel {
  flex: 1;
  padding: 2rem;
}

.setting-section h2 {
  color: #333;
  margin-bottom: 1.5rem;
  font-size: 1.4rem;
}

.setting-group {
  margin-bottom: 2rem;
}

.setting-item {
  margin-bottom: 1.5rem;
}

.setting-item label {
  display: block;
  color: #333;
  font-weight: 500;
  margin-bottom: 0.5rem;
}

.setting-input,
.setting-textarea,
.setting-select {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: border-color 0.3s ease;
}

.setting-input:focus,
.setting-textarea:focus,
.setting-select:focus {
  outline: none;
  border-color: #1890ff;
}

.setting-textarea {
  resize: vertical;
  min-height: 80px;
}

.checkbox-group {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.checkbox-item {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-weight: normal;
  margin-bottom: 0;
}

.checkbox-item input[type='checkbox'] {
  width: auto;
}

.switch-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0;
}

.switch {
  width: auto;
}

.setting-actions {
  display: flex;
  gap: 1rem;
  padding-top: 2rem;
  border-top: 1px solid #f0f0f0;
}

.btn-primary,
.btn-secondary {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: background-color 0.3s ease;
}

.btn-primary {
  background: #1890ff;
  color: white;
}

.btn-primary:hover {
  background: #40a9ff;
}

.btn-secondary {
  background: #f0f0f0;
  color: #666;
}

.btn-secondary:hover {
  background: #e6e6e6;
}
</style>
