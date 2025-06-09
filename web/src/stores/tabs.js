import { defineStore } from 'pinia'

export const useTabStore = defineStore('tabs', {
  state: () => ({
    tabs: [],
  }),

  actions: {
    addTab(tab) {
      const existingTab = this.tabs.find((t) => t.path === tab.path)
      if (!existingTab) {
        this.tabs.push({
          path: tab.path,
          title: tab.title,
          closable: tab.closable,
        })
      }
    },

    removeTab(path) {
      const index = this.tabs.findIndex((tab) => tab.path === path)
      if (index !== -1 && this.tabs[index].closable) {
        this.tabs.splice(index, 1)
      }
    },

    removeAllClosableTabs() {
      this.tabs = this.tabs.filter((tab) => !tab.closable)
    },

    initializeHomePage() {
      const homeTab = {
        path: '/',
        title: '首页',
        closable: false,
      }
      if (!this.tabs.find((tab) => tab.path === '/')) {
        this.tabs.unshift(homeTab)
      }
    },
  },
})
