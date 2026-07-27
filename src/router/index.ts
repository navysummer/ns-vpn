import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('@/pages/_layout.vue'),
      children: [
        { path: '', name: 'home', component: () => import('@/pages/home.vue') },
        { path: 'proxies', name: 'proxies', component: () => import('@/pages/proxies.vue') },
        { path: 'profile', name: 'profile', component: () => import('@/pages/profiles.vue') },
        { path: 'connections', name: 'connections', component: () => import('@/pages/connections.vue') },
        { path: 'rules', name: 'rules', component: () => import('@/pages/rules.vue') },
        { path: 'logs', name: 'logs', component: () => import('@/pages/logs.vue') },
        { path: 'unlock', name: 'unlock', component: () => import('@/pages/unlock.vue') },
        { path: 'settings', name: 'settings', component: () => import('@/pages/settings.vue') },
      ],
    },
  ],
})

export { router }
