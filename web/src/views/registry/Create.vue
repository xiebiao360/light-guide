<script setup>
import { ref } from 'vue'
const create_method = ref('')
</script>

<template>
  <div class="create_container">
    <template v-if="create_method.length === 0">
      <div class="content">
        <a-space>
          <a-button @click="create_method = 'create'">创建镜像仓库</a-button>
          <a-button @click="create_method = 'connect_other'">连接到其他镜像仓库</a-button>
        </a-space>
      </div>
    </template>
    <template v-else-if="create_method === 'create'">
      <a-page-header style="padding: 0px" title="创建镜像仓库" @back="create_method = ''" />
      <div class="content">
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
    </template>
    <template v-else-if="create_method === 'connect_other'">
      <a-page-header style="padding: 0px" title="连接到其他镜像仓库" @back="create_method = ''" />
      <div class="content">
        <a-form>
          <a-form-item label="镜像仓库地址">
            <a-input v-model="address" />
          </a-form-item>
          <a-form-item>
            <a-button type="primary">连接</a-button>
          </a-form-item>
        </a-form>
      </div>
    </template>
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
