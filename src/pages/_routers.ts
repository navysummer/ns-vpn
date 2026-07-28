import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

import Layout from './_layout.vue'
import { navItems } from './_navigation'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: Layout,
    children: navItems.map(
      (item) =>
        ({
          path: item.path,
          component: item.component,
        }) as RouteRecordRaw,
    ),
  },
]

export const router = createRouter({
  history: createWebHistory(),
  routes,
})
