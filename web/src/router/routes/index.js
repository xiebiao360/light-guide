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
        name: 'enviroment',
        path: '/enviroment',
        component: () => import('@/views/enviroment/Index.vue'),
        redirect: '/enviroment/docker',
        children: [
          {
            name: 'docker',
            path: 'docker',
            component: () => import('@/views/enviroment/docker/Index.vue'),
          },
          {
            name: 'kubernetes',
            path: 'kubernetes',
            component: () => import('@/views/enviroment/kubernetes/Index.vue'),
          },
        ],
      },
      {
        name: 'file-manager',
        path: '/file-manager',
        component: () => import('@/views/file-manager/Index.vue'),
      },
      {
        name: 'version',
        path: '/version',
        component: () => import('@/views/version/Index.vue'),
      },
      {
        name: 'install',
        path: '/install',
        component: () => import('@/views/install/Index.vue'),
      },
      {
        name: 'registry',
        path: '/registry',
        component: () => import('@/views/registry/Index.vue'),
        children: [
          {
            name: 'create',
            path: 'create',
            component: () => import('@/views/registry/Create.vue'),
          },
          {
            name: 'error',
            path: 'error',
            component: () => import('@/views/registry/Error.vue'),
          },
        ],
      },
    ],
  },
]

export { routes }
