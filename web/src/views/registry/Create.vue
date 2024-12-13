<script setup>
import { ref } from 'vue'
const create_method = ref('')
</script>

<template>
  <div class="create_container">
    <a-page-header style="padding: 0px" title="创建镜像仓库" />
    <div class="content">
      <div v-if="create_method.length === 0">
        <a-space>
          <a-button @click="create_method = 'create'">创建镜像仓库</a-button>
          <a-button @click="create_method = 'connect_other'">连接到其他镜像仓库</a-button>
        </a-space>
      </div>
      <div v-else-if="create_method === 'create'">
        <a-form>
          <a-form-item label="镜像仓库名称">
            <a-input v-model="name" />
          </a-form-item>
          <a-form-item label="镜像仓库地址">
            <a-input v-model="address" />
          </a-form-item>
          <a-form-item>
            <a-button type="primary">创建</a-button>
          </a-form-item>
        </a-form>
      </div>
      <div v-else-if="create_method === 'connect_other'">
        <a-form>
          <a-form-item label="镜像仓库地址">
            <a-input v-model="address" />
          </a-form-item>
          <a-form-item>
            <a-button type="primary">连接</a-button>
          </a-form-item>
        </a-form>
      </div>
    </div>
  </div>
</template>

<style scoped>
.create_container {
  height: 100%;
  display: flex;
  flex-direction: column;
  padding-top: -20px;
}
.content {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
