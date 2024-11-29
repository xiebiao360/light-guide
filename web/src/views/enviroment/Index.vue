<script setup>
import { useRouter } from 'vue-router'
const router = useRouter()
const navs = [
  { name: 'Docker', path: '/enviroment/docker' },
  { name: 'Kubernetes', path: '/enviroment/kubernetes' },
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
  <div class="header">
    <div v-for="nav in navs" :key="nav.path" :class="isActive(nav.path).active ? 'active' : ''" @click="goto(nav.path)">
      {{ nav.name }}
    </div>
  </div>
  <router-view />
</template>

<style scoped>
.header {
  display: flex;
  justify-content: center;
  align-items: baseline;
  padding: 10px 20px;
  font-size: larger;
  color: cadetblue;
}

.header div {
  padding: 0px 10px;
  cursor: pointer;
}

.active {
  font-size: 1.5em;
  color: #213547;
}
</style>
