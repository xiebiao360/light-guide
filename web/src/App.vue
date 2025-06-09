<script setup>
import { RouterView } from 'vue-router'
</script>

<template>
  <div id="app">
    <RouterView />
  </div>
</template>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body {
  height: 100%;
  font-family:
    -apple-system, BlinkMacSystemFont, 'Segoe UI', 'Roboto', 'Oxygen', 'Ubuntu', 'Cantarell',
    'Fira Sans', 'Droid Sans', 'Helvetica Neue', sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  background-color: #f5f5f5;
}

#app {
  height: 100vh;
  overflow: hidden;
}

/* 自定义滚动条样式 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* 响应式设计 */
/* 大屏幕优化 */
@media (min-width: 1600px) {
  .features-grid {
    grid-template-columns: repeat(4, 1fr) !important;
  }
  
  .metrics-grid {
    grid-template-columns: repeat(4, 1fr) !important;
  }
  
  .charts-section {
    grid-template-columns: repeat(2, 1fr) !important;
  }
  
  .stats-section {
    grid-template-columns: repeat(4, 1fr) !important;
  }
}

@media (min-width: 2000px) {
  .features-grid {
    grid-template-columns: repeat(4, 1fr) !important;
    max-width: 1800px !important;
    margin: 0 auto !important;
  }
  
  .stats-section {
    max-width: 1800px !important;
    margin: 0 auto !important;
  }
  
  .metrics-grid {
    max-width: 1800px !important;
    margin: 0 auto 3rem auto !important;
  }
  
  .charts-section {
    max-width: 1800px !important;
    margin: 0 auto !important;
  }
}
@media (max-width: 768px) {
  .sidebar {
    width: 200px !important;
  }

  .sidebar-header h2 {
    font-size: 1.2rem !important;
  }

  .nav-text {
    font-size: 0.85rem !important;
  }

  .main-content {
    padding: 15px !important;
  }

  .features-grid {
    grid-template-columns: 1fr !important;
  }

  .metrics-grid {
    grid-template-columns: repeat(2, 1fr) !important;
  }

  .charts-section {
    grid-template-columns: 1fr !important;
  }
}

@media (max-width: 480px) {
  .sidebar {
    width: 180px !important;
  }

  .metrics-grid {
    grid-template-columns: 1fr !important;
  }

  .tab-item {
    min-width: 100px !important;
    padding: 6px 12px !important;
  }

  .toolbar {
    flex-direction: column !important;
    align-items: stretch !important;
  }

  .search-box {
    max-width: none !important;
  }
}
</style>
