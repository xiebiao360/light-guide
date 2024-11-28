import HomeView from '@/views/HomeView.vue'

const routes = [
  {
    name: 'home',
    path: '/',
    component: HomeView,
    redirect: '/overview',
    children: [
      {
        name: 'overview',
        path: '/overview',
        component: () => import('@/views/overview/Index.vue'),
      },
      {
        name: 'install',
        path: '/install',
        component: () => import('@/views/install/Index.vue'),
      },
      {
        name: 'file-manager',
        path: '/file-manager',
        component: () => import('@/views/file-manager/Index.vue'),
      },
    ],
  },
]

export { routes }
