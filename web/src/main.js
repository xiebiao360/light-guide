import { createApp } from 'vue'
import { Button, message } from 'ant-design-vue'

import './style.css'

import App from './App.vue'

const app = createApp(App)

app.use(Button)

app.mount('#app')

app.config.globalProperties.$message = message
