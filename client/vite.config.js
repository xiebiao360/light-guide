import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || 'localhost',
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell Vite to ignore watching `src-tauri` and other directories
      ignored: ['**/src-tauri/**', '**/agent/**', '**/shared/**'],
    },
  },

  // 为 Tauri 环境变量设置前缀
  envPrefix: ['VITE_', 'TAURI_'],

  build: {
    // Tauri 在静态构建中使用 Chromium，所以我们可以针对现代浏览器进行构建
    target: 'chrome105',
    // 不最小化调试构建
    minify: 'esbuild',
    // 在调试构建中生成源映射
    sourcemap: true,
  },
})
