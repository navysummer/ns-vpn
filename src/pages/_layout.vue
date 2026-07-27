<template>
  <template v-if="themeReady">
    <NoticeManager :position="verge?.notice_position" />
    <ServiceMigrationDialog />

    <div
      :class="`${OS} layout${navCollapsed ? ' layout--nav-collapsed' : ''}`"
      class="layout-paper"
      @contextmenu="handleContextMenu"
    >
      <WindowResizeHandles v-if="decorated === false" />

      <div v-if="decorated === false" class="the_titlebar">
        <div class="the_titlebar-drag-region" data-tauri-drag-region="true" />
        <WindowControls ref="windowControlsRef" />
      </div>

      <div class="layout-content">
        <div class="layout-content__left">
          <div class="the-logo" data-tauri-drag-region="false">
            <div
              data-tauri-drag-region="true"
              style="height: 27px; display: flex; justify-content: space-between; align-items: center"
            >
              <img
                :src="logoIcon"
                style="height: 36px; width: 36px; margin-top: -3px; margin-right: 5px; margin-left: -3px"
              />
            </div>
            <UpdateButton class="the-newbtn" />
          </div>

          <div
            v-if="menuUnlocked"
            class="menu-reorder-badge"
          >
            {{ t('layout.components.navigation.menu.reorderMode') }}
          </div>

          <div class="the-menu" @contextmenu="handleMenuContextMenu">
            <template v-for="path in menuOrder" :key="path">
              <LayoutItem
                v-if="navItemMap.get(path)"
                :to="navItemMap.get(path)!.path"
                :icon="navItemMap.get(path)!.icon"
              >
                {{ t(navItemMap.get(path)!.label) }}
              </LayoutItem>
            </template>
          </div>

          <n-dropdown
            placement="bottom-start"
            trigger="manual"
            :show="showDropdown"
            :x="menuContextPosition?.left ?? 0"
            :y="menuContextPosition?.top ?? 0"
            :options="dropdownOptions"
            @clickoutside="handleMenuContextClose"
            @select="handleContextMenuSelect"
          />

          <div class="the-traffic">
            <LayoutTraffic />
          </div>
        </div>

        <div class="layout-content__right">
          <div class="the-bar" />
          <div class="the-content">
            <BaseErrorBoundary>
              <router-view />
            </BaseErrorBoundary>
          </div>
        </div>
      </div>
    </div>
  </template>
  <div
    v-else
    :style="{
      width: '100vw',
      height: '100vh',
      background: isDark ? '#181a1b' : '#fff',
      transition: 'background 0.2s',
      display: 'flex',
      alignItems: 'center',
      justifyContent: 'center',
    }"
  />
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useThemeMode } from '@/services/states'
import { useVerge } from '@/hooks/use-verge'
import { useCustomTheme } from './_layout/hooks/use-custom-theme'
import { useLayoutEvents } from './_layout/hooks/use-layout-events'
import { useLoadingOverlay } from './_layout/hooks/use-loading-overlay'
import { useNavMenuOrder } from './_layout/hooks/use-nav-menu-order'
import { handleNoticeMessage } from './_layout/utils/notification-handlers'
import { useI18n } from '@/hooks/use-i18n'
import { useWindowDecorations } from '@/hooks/use-window'
import { useTranslation } from '@/composables/use-i18n'
import getSystem from '@/utils/get-system'
import dayjs from 'dayjs'
import relativeTime from 'dayjs/plugin/relativeTime'
import 'dayjs/locale/ru'
import 'dayjs/locale/zh-cn'

import BaseErrorBoundary from '@/components/base/base-error-boundary.vue'
import LayoutItem from '@/components/layout/layout-item.vue'
import LayoutTraffic from '@/components/layout/layout-traffic.vue'
import NoticeManager from '@/components/layout/notice-manager.vue'
import ServiceMigrationDialog from '@/components/layout/service-migration-dialog.vue'
import UpdateButton from '@/components/layout/update-button.vue'
import WindowControls from '@/components/layout/window-controller.vue'
import WindowResizeHandles from '@/components/layout/window-resize-handles.vue'

import { navItems, type NavItem } from './_navigation'

const { t } = useTranslation()
dayjs.extend(relativeTime)

const OS = getSystem()
const mode = useThemeMode()
const isDark = computed(() => mode.value !== 'light')
const { theme, naiveTheme } = useCustomTheme()
const { verge, mutateVerge, patchVerge } = useVerge()
const { language } = verge ?? {}
const navCollapsed = computed(() => verge?.collapse_navbar ?? false)
const { switchLanguage } = useI18n()
const router = useRouter()
const themeReady = computed(() => Boolean(theme.value))

const menuUnlocked = ref(false)
const menuContextPosition = ref<{ top: number; left: number } | null>(null)
const windowControlsRef = ref<any>(null)
const { decorated } = useWindowDecorations()
const showDropdown = computed(() => Boolean(menuContextPosition.value))

const dropdownOptions = computed(() => [
  {
    key: 'toggle-nav',
    label: navCollapsed.value
      ? t('layout.components.navigation.menu.expandNavBar')
      : t('layout.components.navigation.menu.collapseNavBar'),
  },
  {
    key: 'toggle-lock',
    label: menuUnlocked.value
      ? t('layout.components.navigation.menu.lock')
      : t('layout.components.navigation.menu.unlock'),
  },
  {
    key: 'reset-order',
    label: t('layout.components.navigation.menu.restoreDefaultOrder'),
    disabled: isDefaultOrder.value,
  },
])

const logoIcon = computed(() =>
  isDark.value ? '/src/assets/image/icon_dark.svg' : '/src/assets/image/icon_light.svg',
)

const handleMenuOrderOptimisticUpdate = (order: string[]) => {
  mutateVerge(
    (prev: any) => (prev ? { ...prev, menu_order: order } : prev),
    false,
  )
}

const handleMenuOrderPersist = (order: string[]) => patchVerge({ menu_order: order })

const {
  menuOrder,
  navItemMap,
  handleMenuDragEnd,
  isDefaultOrder,
  resetMenuOrder,
} = useNavMenuOrder({
  enabled: computed(() => menuUnlocked.value),
  items: navItems,
  storedOrder: computed(() => verge?.menu_order),
  onOptimisticUpdate: handleMenuOrderOptimisticUpdate,
  onPersist: handleMenuOrderPersist,
})

const handleMenuContextMenu = (event: MouseEvent) => {
  event.preventDefault()
  event.stopPropagation()
  menuContextPosition.value = { top: event.clientY, left: event.clientX }
}

const handleMenuContextClose = () => {
  menuContextPosition.value = null
}

const handleContextMenuSelect = (key: string | number) => {
  const optionKey = String(key)
  menuContextPosition.value = null
  if (optionKey === 'toggle-nav') {
    void patchVerge({ collapse_navbar: !navCollapsed.value })
  } else if (optionKey === 'toggle-lock') {
    menuUnlocked.value = !menuUnlocked.value
  } else if (optionKey === 'reset-order') {
    void resetMenuOrder()
  }
}

useLoadingOverlay(themeReady)
useLayoutEvents((payload: [string, string]) => {
  const [status, msg] = payload
  try {
    handleNoticeMessage(status, msg, t, (path: string) => router.push(path))
  } catch (error) {
    console.error('[通知处理] 失败:', error)
  }
})

watch(language, (lang) => {
  if (lang) {
    dayjs.locale(lang === 'zh' ? 'zh-cn' : lang)
    switchLanguage(lang)
  }
}, { immediate: true })
</script>

<style scoped>
.layout-paper {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background-color: var(--background-color);
}

.layout-paper.linux {
  border-radius: 8px;
  width: 100vw;
  height: 100vh;
}

.the-menu {
  flex: 1;
  overflow-y: auto;
}

.menu-reorder-badge {
  padding: 6px 12px;
  margin: 0 auto 8px;
  max-width: 250px;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 600;
  text-align: center;
  background-color: var(--warning-color);
  color: #fff;
}
</style>
