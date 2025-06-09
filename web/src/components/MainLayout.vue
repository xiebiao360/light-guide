<template>
  <div class="main-layout">
    <Sidebar />
    <div class="content-wrapper">
      <ContentArea />
    </div>
  </div>
</template>

<script setup>
import Sidebar from './Sidebar.vue'
import ContentArea from './ContentArea.vue'
</script>

<style scoped>
.main-layout {
  display: flex;
  height: 100vh;
  background-color: #f5f5f5;
}

.content-wrapper {
  flex: 1;
  overflow: hidden;
}
</style>
