<template>
  <div class="content-area">
    <Header />
    <TabManager />
    <div class="main-content">
      <router-view />
    </div>
  </div>
</template>

<script setup>
import Header from './Header.vue'
import TabManager from './TabManager.vue'
</script>

<style scoped>
.content-area {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: white;
}

.main-content {
  flex: 1;
  padding: 20px 0;
  overflow-y: auto;
  background: #fafafa;
}
</style>
