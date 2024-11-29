import { createApp } from 'vue'
import { Button, Table, message } from 'ant-design-vue'

import './style.css'
import { router } from './router'

import App from './App.vue'

const app = createApp(App)

app.use(router)

app.use(Button)

app.mount('#app')

app.config.globalProperties.$message = message
