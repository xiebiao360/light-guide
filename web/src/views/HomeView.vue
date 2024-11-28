<script setup>
import { useRouter } from 'vue-router'
const router = useRouter()
const navs = [
  { name: '概览', path: '/overview' },
  { name: '资源管理', path: '/file-manager' },
  { name: '一键安装', path: '/install' },
]
const goto = path => router.push(path)

const active = path => {
  return router.currentRoute.value.path === path
}

const isActive = path => {
  return {
    active: active(path),
  }
}
</script>

<template>
  <div class="container">
    <div class="sider">
      <div v-for="nav in navs" :key="nav.path" :class="isActive(nav.path).active ? 'sider-item active' : 'sider-item'" @click="goto(nav.path)">
        {{ nav.name }}
      </div>
    </div>
    <div class="devider"></div>
    <div class="content">
      <router-view />
    </div>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  align-items: center;
  height: 100%;
}
.sider {
  width: 20%;
  min-width: 120px;
  height: 100%;
  display: flex;
  flex-direction: column;
}
.sider-item {
  padding: 10px 0px;
  cursor: pointer;
}
.sider-item:first-child {
  border-top-left-radius: 12px;
}

.devider {
  width: 1px;
  height: 100%;
  background-color: #e0e0e0;
}
.content {
  flex: 1;
  height: 100%;
  padding: 2rem;
  box-sizing: border-box;
}
.active {
  background-color: #f0f0f0;
}
</style>
