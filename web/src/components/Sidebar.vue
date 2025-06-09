<template>
  <div class="sidebar">
    <div class="sidebar-header">
      <h2>Light Guide</h2>
    </div>
    <nav class="sidebar-nav">
      <ul>
        <li v-for="menu in menuItems" :key="menu.path">
          <a
            @click="handleMenuClick(menu)"
            :class="{ active: $route.path === menu.path }"
            class="nav-link"
          >
            <span class="nav-icon">{{ menu.icon }}</span>
            <span class="nav-text">{{ menu.title }}</span>
          </a>
        </li>
      </ul>
    </nav>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { useTabStore } from '@/stores/tabs'

const router = useRouter()
const tabStore = useTabStore()

const menuItems = [
  { path: '/', title: '首页', icon: '🏠', closable: false },
  { path: '/dashboard', title: '仪表板', icon: '📊', closable: true },
  { path: '/users', title: '用户管理', icon: '👥', closable: true },
  { path: '/settings', title: '系统设置', icon: '⚙️', closable: true },
  { path: '/about', title: '关于', icon: 'ℹ️', closable: true },
]

const handleMenuClick = (menu) => {
  tabStore.addTab({
    path: menu.path,
    title: menu.title,
    closable: menu.closable,
  })
  router.push(menu.path)
}
</script>

<style scoped>
.sidebar {
  width: 260px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  flex-direction: column;
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.1);
}

.sidebar-header {
  padding: 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.1);
}

.sidebar-header h2 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: white;
}

.sidebar-nav {
  flex: 1;
  padding: 20px 0;
}

.sidebar-nav ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

.sidebar-nav li {
  margin-bottom: 4px;
}

.nav-link {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  color: rgba(255, 255, 255, 0.8);
  text-decoration: none;
  transition: all 0.3s ease;
  cursor: pointer;
  border-radius: 0 25px 25px 0;
  margin-right: 10px;
}

.nav-link:hover {
  background: rgba(255, 255, 255, 0.1);
  color: white;
  transform: translateX(5px);
}

.nav-link.active {
  background: rgba(255, 255, 255, 0.2);
  color: white;
  transform: translateX(5px);
}

.nav-icon {
  margin-right: 12px;
  font-size: 1.2rem;
}

.nav-text {
  font-size: 0.95rem;
  font-weight: 500;
}
</style>
