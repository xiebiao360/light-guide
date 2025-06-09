<template>
  <div class="tab-manager" v-if="tabStore.tabs.length > 0">
    <div class="tabs-container">
      <div
        v-for="tab in tabStore.tabs"
        :key="tab.path"
        :class="['tab-item', { active: $route.path === tab.path }]"
        @click="switchTab(tab.path)"
      >
        <span class="tab-title">{{ tab.title }}</span>
        <button
          v-if="tab.closable"
          @click.stop="closeTab(tab.path)"
          class="close-btn"
          title="关闭标签页"
        >
          ×
        </button>
      </div>
    </div>
    <div class="tab-actions">
      <button @click="closeAllTabs" class="action-btn" title="关闭所有可关闭标签页">
        关闭所有
      </button>
    </div>
  </div>
</template>

<script setup>
import { useRouter } from 'vue-router'
import { useTabStore } from '@/stores/tabs'

const router = useRouter()
const tabStore = useTabStore()

const switchTab = (path) => {
  router.push(path)
}

const closeTab = (path) => {
  const wasActive = router.currentRoute.value.path === path
  tabStore.removeTab(path)

  if (wasActive && tabStore.tabs.length > 0) {
    const lastTab = tabStore.tabs[tabStore.tabs.length - 1]
    router.push(lastTab.path)
  }
}

const closeAllTabs = () => {
  const homeTab = tabStore.tabs.find((tab) => !tab.closable)
  tabStore.removeAllClosableTabs()

  if (homeTab && router.currentRoute.value.path !== homeTab.path) {
    router.push(homeTab.path)
  }
}
</script>

<style scoped>
.tab-manager {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background: #f8f8f8;
  border-bottom: 1px solid #e8e8e8;
  padding: 0 16px;
  min-height: 42px;
}

.tabs-container {
  display: flex;
  align-items: center;
  flex: 1;
  overflow-x: auto;
  gap: 2px;
}

.tab-item {
  display: flex;
  align-items: center;
  padding: 8px 16px;
  background: white;
  border: 1px solid #d9d9d9;
  border-bottom: none;
  border-radius: 6px 6px 0 0;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
  min-width: 120px;
  position: relative;
}

.tab-item:hover {
  background: #f0f0f0;
}

.tab-item.active {
  background: white;
  border-color: #1890ff;
  color: #1890ff;
  box-shadow: 0 -2px 4px rgba(0, 0, 0, 0.1);
}

.tab-title {
  font-size: 0.9rem;
  margin-right: 8px;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
}

.close-btn {
  background: none;
  border: none;
  color: #999;
  font-size: 18px;
  cursor: pointer;
  padding: 0;
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 3px;
  transition: all 0.2s ease;
}

.close-btn:hover {
  background: #ff4d4f;
  color: white;
}

.tab-actions {
  margin-left: 16px;
}

.action-btn {
  padding: 4px 12px;
  background: #f0f0f0;
  border: 1px solid #d9d9d9;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  color: #666;
  transition: all 0.2s ease;
}

.action-btn:hover {
  background: #e6e6e6;
  color: #333;
}

.tabs-container::-webkit-scrollbar {
  height: 4px;
}

.tabs-container::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.tabs-container::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 2px;
}

.tabs-container::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style>
