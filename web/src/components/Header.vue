<template>
  <header class="header">
    <div class="header-left">
      <h1 class="page-title">{{ currentPageTitle }}</h1>
    </div>
    <div class="header-right">
      <div class="user-info">
        <span class="username">用户名</span>
        <div class="avatar">👤</div>
      </div>
    </div>
  </header>
</template>

<script setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useTabStore } from '@/stores/tabs'

const route = useRoute()
const tabStore = useTabStore()

const currentPageTitle = computed(() => {
  const currentTab = tabStore.tabs.find((tab) => tab.path === route.path)
  return currentTab?.title || '页面'
})
</script>

<style scoped>
.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0 24px;
  height: 64px;
  background: white;
  border-bottom: 1px solid #e8e8e8;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.header-left {
  display: flex;
  align-items: center;
}

.page-title {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
  color: #333;
}

.header-right {
  display: flex;
  align-items: center;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.username {
  font-size: 0.9rem;
  color: #666;
}

.avatar {
  width: 36px;
  height: 36px;
  background: #f0f0f0;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.2rem;
  cursor: pointer;
  transition: background-color 0.3s ease;
}

.avatar:hover {
  background: #e6e6e6;
}
</style>
