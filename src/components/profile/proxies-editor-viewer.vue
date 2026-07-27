<template>
  <n-modal
    :show="open"
    @update:show="onCloseWrapper"
    :mask-closable="false"
    preset="card"
    style="max-width: 90vw; width: 100%;"
    :segmented="{ content: true }"
  >
    <template #header>
      <div style="display: flex; justify-content: space-between; align-items: center; width: 100%;">
        <span>{{ t('profiles.modals.proxiesEditor.title') }}</span>
        <n-button size="small" @click="setVisualization(!visualization)">
          {{ visualization ? t('shared.editorModes.advanced') : t('shared.editorModes.visualization') }}
        </n-button>
      </div>
    </template>
    <div style="display: flex; width: auto; height: calc(100vh - 185px);">
      <template v-if="visualization">
        <div style="width: 50%; padding: 0 10px;">
          <div style="height: calc(100% - 80px); overflow-y: auto;">
            <div style="padding: 5px 2px;">
              <textarea
                :placeholder="t('profiles.modals.proxiesEditor.placeholders.multiUri')"
                style="width: 100%; min-height: 180px; resize: vertical; font-family: monospace; padding: 8px; box-sizing: border-box; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
                @input="proxyUri = ($event.target as HTMLTextAreaElement).value"
              />
            </div>
          </div>
          <div style="padding: 5px 2px;">
            <n-button block @click="handleParseAsync((proxies) => { prependSeq = [...proxies, ...prependSeq] })">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M8 11h3v10h2V11h3l-4-4-4 4zM4 3v2h16V3H4z"/></svg>
              </template>
              {{ t('profiles.modals.proxiesEditor.actions.prepend') }}
            </n-button>
          </div>
          <div style="padding: 5px 2px;">
            <n-button block @click="handleParseAsync((proxies) => { appendSeq = [...appendSeq, ...proxies] })">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M16 13h-3V3h-2v10H8l4 4 4-4zM4 19v2h16v-2H4z"/></svg>
              </template>
              {{ t('profiles.modals.proxiesEditor.actions.append') }}
            </n-button>
          </div>
        </div>
        <div style="width: 50%; padding: 0 10px;">
          <div>
            <input
              type="text"
              :placeholder="t('shared.search')"
              style="width: 100%; padding: 6px 12px; border: 1px solid var(--border-color); border-radius: 4px; box-sizing: border-box; background: var(--bg-color); color: var(--text-color);"
              @input="onSearch(($event.target as HTMLInputElement).value)"
            />
          </div>
          <div style="height: calc(100% - 24px); margin-top: 8px; overflow-y: auto;">
            <template v-if="filteredPrependSeq.length > 0">
              <ProxyItem
                v-for="item in filteredPrependSeq"
                :key="item.name"
                type="prepend"
                :proxy="item"
                @delete="prependSeq = prependSeq.filter(v => v.name !== item.name)"
              />
            </template>
            <ProxyItem
              v-for="proxy in filteredProxyList"
              :key="proxy.name"
              :type="deleteSeq.includes(proxy.name) ? 'delete' : 'original'"
              :proxy="proxy"
              @delete="deleteSeq = deleteSeq.includes(proxy.name) ? deleteSeq.filter(v => v !== proxy.name) : [...deleteSeq, proxy.name]"
            />
            <ProxyItem
              v-for="item in filteredAppendSeq"
              :key="item.name"
              type="append"
              :proxy="item"
              @delete="appendSeq = appendSeq.filter(v => v.name !== item.name)"
            />
          </div>
        </div>
      </template>
      <MonacoEditor
        v-else
        height="100%"
        language="yaml"
        :value="currData"
        :theme="themeMode === 'light' ? 'light' : 'vs-dark'"
        @mount="(instance) => { editorRef = instance }"
        @change="(value) => { currData = value ?? '' }"
        :options="monacoOptions"
      />
    </div>
    <template #footer>
      <div style="display: flex; justify-content: flex-end; gap: 8px; width: 100%;">
        <n-button @click="onClose" quaternary>{{ t('shared.actions.cancel') }}</n-button>
        <n-button @click="handleSave">{{ t('shared.actions.save') }}</n-button>
      </div>
    </template>
  </n-modal>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'
import yaml from 'js-yaml'
import { BaseSearchBox, MonacoEditor } from '@/components/base'
import ProxyItem from '@/components/profile/proxy-item.vue'
import { readProfileFile, saveProfileFile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { useThemeMode } from '@/services/states'
import type { MonacoEditorInstance } from '@/types/monaco'
import getSystem from '@/utils/get-system'
import parseUri from '@/utils/uri-parser'

const props = defineProps<{
  profileUid: string
  property: string
  open: boolean
  onClose: () => void
  onSave?: (prev?: string, curr?: string) => void
}>()

const emit = defineEmits<{
  save: [prev?: string, curr?: string]
  close: []
}>()

const { t } = useI18n()
const themeMode = useThemeMode()
const editorRef = ref<MonacoEditorInstance | null>(null)
const prevData = ref('')
const currData = ref('')
const visualization = ref(true)
const matchText = ref('')
const proxyUri = ref('')

const proxyList = ref<IProxyConfig[]>([])
const prependSeq = ref<IProxyConfig[]>([])
const appendSeq = ref<IProxyConfig[]>([])
const deleteSeq = ref<string[]>([])

const hasValidName = (proxy: IProxyConfig) => typeof proxy?.name === 'string' && proxy.name.length > 0
const matches = (name: string) => !matchText.value || name.toLowerCase().includes(matchText.value.toLowerCase())

const filteredPrependSeq = computed(() => prependSeq.value.filter(p => hasValidName(p) && matches(p.name)))
const filteredProxyList = computed(() => proxyList.value.filter(p => hasValidName(p) && matches(p.name)))
const filteredAppendSeq = computed(() => appendSeq.value.filter(p => hasValidName(p) && matches(p.name)))

const onSearch = (text: string) => { matchText.value = text }

const handleParseAsync = (cb: (proxies: IProxyConfig[]) => void) => {
  const proxies: IProxyConfig[] = []
  const names: string[] = []
  let uris: string
  try { uris = atob(proxyUri.value) } catch { uris = proxyUri.value }
  const lines = uris.trim().split('\n')
  let idx = 0
  const batchSize = 50
  const parseNext = () => {
    const end = Math.min(idx + batchSize, lines.length)
    for (; idx < end; idx++) {
      try {
        const p = parseUri(lines[idx].trim())
        if (!names.includes(p.name)) { proxies.push(p); names.push(p.name) }
      } catch { /* ignore */ }
    }
    if (idx < lines.length) setTimeout(parseNext, 0)
    else cb(proxies)
  }
  parseNext()
}

const fetchContent = async () => {
  const data = await readProfileFile(props.property)
  const obj = yaml.load(data) as ISeqProfileConfig | null
  prependSeq.value = obj?.prepend || []
  appendSeq.value = obj?.append || []
  deleteSeq.value = obj?.delete || []
  prevData.value = data
  currData.value = data
}

const fetchProfile = async () => {
  const data = await readProfileFile(props.profileUid)
  const obj = yaml.load(data) as { proxies: IProxyConfig[] } | null
  proxyList.value = obj?.proxies || []
}

watch(currData, (val) => {
  if (!val || !visualization.value) return
  const obj = yaml.load(val) as ISeqProfileConfig | null
  prependSeq.value = obj?.prepend ?? []
  appendSeq.value = obj?.append ?? []
  deleteSeq.value = obj?.delete ?? []
})

watch([prependSeq, appendSeq, deleteSeq], () => {
  const serialize = () => {
    try {
      currData.value = yaml.dump(
        { prepend: prependSeq.value, append: appendSeq.value, delete: deleteSeq.value },
        { forceQuotes: true },
      )
    } catch { /* ignore */ }
  }
  let idleId: number | undefined
  let timeoutId: number | undefined
  if (window.requestIdleCallback) {
    idleId = window.requestIdleCallback(serialize)
  } else {
    timeoutId = window.setTimeout(serialize, 0)
  }
  onUnmounted(() => {
    if (idleId !== undefined && window.cancelIdleCallback) window.cancelIdleCallback(idleId)
    if (timeoutId !== undefined) clearTimeout(timeoutId)
  })
})

watch(() => props.open, (val) => { if (val) { fetchContent(); fetchProfile() } })

const onCloseWrapper = (val: boolean) => { if (!val) props.onClose() }

const monacoOptions = computed(() => ({
  tabSize: 2,
  minimap: { enabled: document.documentElement.clientWidth >= 1500 },
  mouseWheelZoom: true,
  quickSuggestions: { strings: true, comments: true, other: true },
  padding: { top: 33 },
  fontFamily: `Fira Code, JetBrains Mono, Roboto Mono, "Source Code Pro", Consolas, Menlo, Monaco, monospace, "Courier New", "Apple Color Emoji"${getSystem() === 'windows' ? ', twemoji mozilla' : ''}`,
  fontLigatures: false,
  smoothScrolling: true,
}))

let saveLock = false
const handleSave = async () => {
  if (saveLock) return
  saveLock = true
  try {
    if (!(await saveProfileFile(props.property, currData.value))) { await fetchContent(); props.onClose(); return }
    showNotice.success('shared.feedback.notifications.saved')
    props.onSave?.(prevData.value, currData.value)
    props.onClose()
  } catch (err) { showNotice.error(err) }
  finally { saveLock = false }
}
</script>
