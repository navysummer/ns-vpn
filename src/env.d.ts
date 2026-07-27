/// <reference types="vite/client" />

declare module '*.vue' {
  import type { DefineComponent } from 'vue'
  const component: DefineComponent<{}, {}, any>
  export default component
}

declare module '*.svg?react' {
  import type { FunctionalComponent, SVGAttributes } from 'vue'
  const component: FunctionalComponent<SVGAttributes>
  export default component
}

declare module '*.svg' {
  const content: string
  export default content
}

interface Window {
  __VERGE_INITIAL_THEME_MODE?: 'light' | 'dark' | 'system'
}
