import { createApp } from 'vue'
import { Button, Table, Alert, Space, Upload, message } from 'ant-design-vue'

import './style.css'
import { router } from './router'

import App from './App.vue'

const app = createApp(App)

app.use(router)

app.use(Button).use(Table).use(Alert).use(Space).use(Upload)

app.mount('#app')

app.config.globalProperties.$message = message
