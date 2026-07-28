import { defineComponent, h, type Component } from 'vue'

import {
  HomeOutline,
  WifiOutline,
  ServerOutline,
  GlobeOutline,
  GitBranchOutline,
  DocumentTextOutline,
  LockOpenOutline,
  SettingsOutline,
} from '@vicons/ionicons5'

import ConnectionsSvg from '@/assets/image/itemicon/connections.svg?raw'
import HomeSvg from '@/assets/image/itemicon/home.svg?raw'
import LogsSvg from '@/assets/image/itemicon/logs.svg?raw'
import ProfilesSvg from '@/assets/image/itemicon/profiles.svg?raw'
import ProxiesSvg from '@/assets/image/itemicon/proxies.svg?raw'
import RulesSvg from '@/assets/image/itemicon/rules.svg?raw'
import SettingsSvg from '@/assets/image/itemicon/settings.svg?raw'
import UnlockSvg from '@/assets/image/itemicon/unlock.svg?raw'

import { navigationItems } from './_navigation-meta'
import ConnectionsPage from './connections.vue'
import HomePage from './home.vue'
import LogsPage from './logs.vue'
import ProfilePage from './profiles.vue'
import ProxyPage from './proxies.vue'
import RulesPage from './rules.vue'
import SettingPage from './settings.vue'
import UnlockPage from './unlock.vue'

function rawSvg(svg: string): Component {
  const html = svg.replace(/<svg[^>]*>/, '').replace(/<\/svg>/, '')
  return defineComponent({
    render() {
      return h('span', { innerHTML: html })
    },
  })
}

export interface NavItem {
  label: (typeof navigationItems)[keyof typeof navigationItems]['label']
  path: string
  icon: Component[]
  component: Component
}

export const navItems: NavItem[] = [
  {
    ...navigationItems.home,
    icon: [HomeOutline, rawSvg(HomeSvg)],
    component: HomePage,
  },
  {
    ...navigationItems.proxies,
    icon: [WifiOutline, rawSvg(ProxiesSvg)],
    component: ProxyPage,
  },
  {
    ...navigationItems.profiles,
    icon: [ServerOutline, rawSvg(ProfilesSvg)],
    component: ProfilePage,
  },
  {
    ...navigationItems.connections,
    icon: [GlobeOutline, rawSvg(ConnectionsSvg)],
    component: ConnectionsPage,
  },
  {
    ...navigationItems.rules,
    icon: [GitBranchOutline, rawSvg(RulesSvg)],
    component: RulesPage,
  },
  {
    ...navigationItems.logs,
    icon: [DocumentTextOutline, rawSvg(LogsSvg)],
    component: LogsPage,
  },
  {
    ...navigationItems.unlock,
    icon: [LockOpenOutline, rawSvg(UnlockSvg)],
    component: UnlockPage,
  },
  {
    ...navigationItems.settings,
    icon: [SettingsOutline, rawSvg(SettingsSvg)],
    component: SettingPage,
  },
]
