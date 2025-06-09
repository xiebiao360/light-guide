<template>
  <div class="users-view">
    <div class="page-header">
      <h1>👥 用户管理</h1>
      <p>管理系统用户信息和权限设置</p>
    </div>

    <div class="toolbar">
      <div class="search-box">
        <input type="text" placeholder="搜索用户..." v-model="searchQuery" class="search-input" />
      </div>
      <button class="btn-primary">+ 添加用户</button>
    </div>

    <div class="users-table">
      <table>
        <thead>
          <tr>
            <th>用户名</th>
            <th>邮箱</th>
            <th>角色</th>
            <th>状态</th>
            <th>注册时间</th>
            <th>操作</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="user in filteredUsers" :key="user.id">
            <td>
              <div class="user-info">
                <div class="avatar">{{ user.name.charAt(0) }}</div>
                <div>
                  <div class="user-name">{{ user.name }}</div>
                  <div class="user-id">ID: {{ user.id }}</div>
                </div>
              </div>
            </td>
            <td>{{ user.email }}</td>
            <td>
              <span :class="['role-badge', user.role]">{{ getRoleText(user.role) }}</span>
            </td>
            <td>
              <span :class="['status-badge', user.status]">{{ getStatusText(user.status) }}</span>
            </td>
            <td>{{ formatDate(user.createdAt) }}</td>
            <td>
              <div class="actions">
                <button class="btn-edit">编辑</button>
                <button class="btn-delete">删除</button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const searchQuery = ref('')

const users = ref([
  {
    id: 1,
    name: '张三',
    email: 'zhangsan@example.com',
    role: 'admin',
    status: 'active',
    createdAt: '2024-01-15',
  },
  {
    id: 2,
    name: '李四',
    email: 'lisi@example.com',
    role: 'user',
    status: 'active',
    createdAt: '2024-02-20',
  },
  {
    id: 3,
    name: '王五',
    email: 'wangwu@example.com',
    role: 'editor',
    status: 'inactive',
    createdAt: '2024-03-10',
  },
  {
    id: 4,
    name: '赵六',
    email: 'zhaoliu@example.com',
    role: 'user',
    status: 'active',
    createdAt: '2024-03-25',
  },
])

const filteredUsers = computed(() => {
  if (!searchQuery.value) return users.value
  return users.value.filter(
    (user) =>
      user.name.toLowerCase().includes(searchQuery.value.toLowerCase()) ||
      user.email.toLowerCase().includes(searchQuery.value.toLowerCase()),
  )
})

const getRoleText = (role) => {
  const roleMap = {
    admin: '管理员',
    editor: '编辑',
    user: '普通用户',
  }
  return roleMap[role] || role
}

const getStatusText = (status) => {
  const statusMap = {
    active: '活跃',
    inactive: '非活跃',
  }
  return statusMap[status] || status
}

const formatDate = (dateStr) => {
  return new Date(dateStr).toLocaleDateString('zh-CN')
}
</script>

<style scoped>
.users-view {
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

.toolbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 2rem;
  gap: 1rem;
}

.search-box {
  flex: 1;
  max-width: 300px;
}

.search-input {
  width: 100%;
  padding: 0.75rem 1rem;
  border: 1px solid #d9d9d9;
  border-radius: 6px;
  font-size: 0.9rem;
  transition: border-color 0.3s ease;
}

.search-input:focus {
  outline: none;
  border-color: #1890ff;
}

.btn-primary {
  padding: 0.75rem 1.5rem;
  background: #1890ff;
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-weight: 500;
  transition: background-color 0.3s ease;
}

.btn-primary:hover {
  background: #40a9ff;
}

.users-table {
  background: white;
  border-radius: 12px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
  overflow: hidden;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th,
td {
  padding: 1rem;
  text-align: left;
  border-bottom: 1px solid #f0f0f0;
}

th {
  background: #fafafa;
  font-weight: 600;
  color: #333;
  font-size: 0.9rem;
}

.user-info {
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.avatar {
  width: 40px;
  height: 40px;
  background: #1890ff;
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 1.1rem;
}

.user-name {
  font-weight: 500;
  color: #333;
}

.user-id {
  font-size: 0.8rem;
  color: #999;
}

.role-badge,
.status-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.8rem;
  font-weight: 500;
}

.role-badge.admin {
  background: #fff7e6;
  color: #fa8c16;
}

.role-badge.editor {
  background: #f6ffed;
  color: #52c41a;
}

.role-badge.user {
  background: #f0f0f0;
  color: #666;
}

.status-badge.active {
  background: #f6ffed;
  color: #52c41a;
}

.status-badge.inactive {
  background: #fff2f0;
  color: #ff4d4f;
}

.actions {
  display: flex;
  gap: 0.5rem;
}

.btn-edit,
.btn-delete {
  padding: 0.4rem 0.8rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  transition: background-color 0.3s ease;
}

.btn-edit {
  background: #e6f7ff;
  color: #1890ff;
}

.btn-edit:hover {
  background: #bae7ff;
}

.btn-delete {
  background: #fff2f0;
  color: #ff4d4f;
}

.btn-delete:hover {
  background: #ffccc7;
}
</style>
