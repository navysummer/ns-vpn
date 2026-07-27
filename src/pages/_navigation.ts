import { h, defineComponent } from 'vue'
import {
  HomeOutline,
  WifiOutline,
  ServerOutline,
  GlobeOutline,
  GitNetworkOutline,
  ListOutline,
  LockOpenOutline,
  SettingsOutline,
} from '@vicons/ionicons5'

import { navigationItems } from './_navigation-meta'
import type { Component } from 'vue'

const createIconComponent = (icon: any) =>
  defineComponent({
    setup() {
      return () => h(icon)
    },
  })

export type NavigationItem = {
  label: (typeof navigationItems)[keyof typeof navigationItems]['label']
  path: string
  icon: Component[]
  Component: any
}

export const navItems: NavigationItem[] = [
  {
    ...navigationItems.home,
    icon: [createIconComponent(HomeOutline), createIconComponent(HomeOutline)],
    Component: () => import('./home.vue'),
  },
  {
    ...navigationItems.proxies,
    icon: [createIconComponent(WifiOutline), createIconComponent(WifiOutline)],
    Component: () => import('./proxies.vue'),
  },
  {
    ...navigationItems.profiles,
    icon: [createIconComponent(ServerOutline), createIconComponent(ServerOutline)],
    Component: () => import('./profiles.vue'),
  },
  {
    ...navigationItems.connections,
    icon: [createIconComponent(GlobeOutline), createIconComponent(GlobeOutline)],
    Component: () => import('./connections.vue'),
  },
  {
    ...navigationItems.rules,
    icon: [createIconComponent(GitNetworkOutline), createIconComponent(GitNetworkOutline)],
    Component: () => import('./rules.vue'),
  },
  {
    ...navigationItems.logs,
    icon: [createIconComponent(ListOutline), createIconComponent(ListOutline)],
    Component: () => import('./logs.vue'),
  },
  {
    ...navigationItems.unlock,
    icon: [createIconComponent(LockOpenOutline), createIconComponent(LockOpenOutline)],
    Component: () => import('./unlock.vue'),
  },
  {
    ...navigationItems.settings,
    icon: [createIconComponent(SettingsOutline), createIconComponent(SettingsOutline)],
    Component: () => import('./settings.vue'),
  },
]
