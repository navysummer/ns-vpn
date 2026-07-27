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
        <span>{{ t('profiles.modals.groupsEditor.title') }}</span>
        <n-button size="small" @click="setVisualization(!visualization)">
          {{ visualization ? t('shared.editorModes.advanced') : t('shared.editorModes.visualization') }}
        </n-button>
      </div>
    </template>
    <div style="display: flex; width: auto; height: calc(100vh - 185px);">
      <template v-if="visualization">
        <div style="width: 50%; padding: 0 10px;">
          <div style="height: calc(100% - 80px); overflow-y: auto;">
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.type') }}</span>
              <select
                v-model="formType.value"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="opt in strategyOptions" :key="opt" :value="opt">{{ translateStrategy(opt) }}</option>
              </select>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.name') }}</span>
              <input
                v-model="formName.value"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
                :class="{ error: formName.value === '' }"
                required
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.icon') }}</span>
              <input
                v-model="formIcon.value"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.proxies') }}</span>
              <select
                multiple
                v-model="formProxies.value"
                style="width: calc(100% - 150px); min-height: 60px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="policy in proxyPolicyList" :key="policy" :value="policy">{{ translatePolicy(policy) }}</option>
              </select>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.provider') }}</span>
              <select
                multiple
                v-model="formUse.value"
                style="width: calc(100% - 150px); min-height: 60px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="provider in proxyProviderList" :key="provider" :value="provider">{{ provider }}</option>
              </select>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.healthCheckUrl') }}</span>
              <input
                v-model="formUrl.value"
                placeholder="http://cp.cloudflare.com/generate_204"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.expectedStatus') }}</span>
              <input
                placeholder="*"
                type="number"
                :value="formExpectedStatus"
                @input="formExpectedStatus = parseInt(($event.target as HTMLInputElement).value) || undefined"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.interval') }}</span>
              <div style="width: calc(100% - 150px); display: flex; align-items: center;">
                <input
                  placeholder="300"
                  type="number"
                  :value="formInterval"
                  @input="formInterval = parseInt(($event.target as HTMLInputElement).value) || 300"
                  style="flex: 1; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
                />
                <span style="margin-left: 4px; font-size: 12px; white-space: nowrap;">{{ t('shared.units.seconds') }}</span>
              </div>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('shared.labels.timeout') }}</span>
              <div style="width: calc(100% - 150px); display: flex; align-items: center;">
                <input
                  placeholder="5000"
                  type="number"
                  :value="formTimeout"
                  @input="formTimeout = parseInt(($event.target as HTMLInputElement).value) || 5000"
                  style="flex: 1; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
                />
                <span style="margin-left: 4px; font-size: 12px; white-space: nowrap;">{{ t('shared.units.milliseconds') }}</span>
              </div>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.maxFailedTimes') }}</span>
              <input
                placeholder="5"
                type="number"
                :value="formMaxFailedTimes"
                @input="formMaxFailedTimes = parseInt(($event.target as HTMLInputElement).value) || 5"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.interfaceName') }}</span>
              <select
                v-model="formInterfaceName.value"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="iface in interfaceNameList" :key="iface" :value="iface">{{ iface }}</option>
              </select>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.routingMark') }}</span>
              <input
                type="number"
                :value="formRoutingMark"
                @input="formRoutingMark = parseInt(($event.target as HTMLInputElement).value) || undefined"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.filter') }}</span>
              <input
                v-model="formFilter.value"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.excludeFilter') }}</span>
              <input
                v-model="formExcludeFilter.value"
                style="width: calc(100% - 150px); padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              />
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.excludeType') }}</span>
              <select
                multiple
                v-model="formExcludeType.value"
                style="width: calc(100% - 150px); min-height: 60px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="opt in excludeTypeOptions" :key="opt" :value="opt">{{ opt }}</option>
              </select>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.includeAll') }}</span>
              <label style="width: calc(100% - 150px);">
                <input type="checkbox" v-model="formIncludeAll.value" />
              </label>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.includeAllProxies') }}</span>
              <label style="width: calc(100% - 150px);">
                <input type="checkbox" v-model="formIncludeAllProxies.value" />
              </label>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.fields.includeAllProviders') }}</span>
              <label style="width: calc(100% - 150px);">
                <input type="checkbox" v-model="formIncludeAllProviders.value" />
              </label>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.toggles.lazy') }}</span>
              <label style="width: calc(100% - 150px);">
                <input type="checkbox" v-model="formLazy.value" />
              </label>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.toggles.disableUdp') }}</span>
              <label style="width: calc(100% - 150px);">
                <input type="checkbox" v-model="formDisableUdp.value" />
              </label>
            </div>
            <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
              <span style="font-size: 14px; font-weight: 500;">{{ t('profiles.modals.groupsEditor.toggles.hidden') }}</span>
              <label style="width: calc(100% - 150px);">
                <input type="checkbox" v-model="formHidden.value" />
              </label>
            </div>
          </div>
          <div style="padding: 5px 2px;">
            <n-button block @click="tryPrepend">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M8 11h3v10h2V11h3l-4-4-4 4zM4 3v2h16V3H4z"/></svg>
              </template>
              {{ t('profiles.modals.groupsEditor.actions.prepend') }}
            </n-button>
          </div>
          <div style="padding: 5px 2px;">
            <n-button block @click="tryAppend">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M16 13h-3V3h-2v10H8l4 4 4-4zM4 19v2h16v-2H4z"/></svg>
              </template>
              {{ t('profiles.modals.groupsEditor.actions.append') }}
            </n-button>
          </div>
        </div>
        <div style="width: 50%; padding: 0 10px;">
          <input
            type="text"
            :placeholder="t('shared.search')"
            style="width: 100%; padding: 6px 12px; border: 1px solid var(--border-color); border-radius: 4px; box-sizing: border-box; background: var(--bg-color); color: var(--text-color);"
            @input="onSearch(($event.target as HTMLInputElement).value)"
          />
          <div style="height: calc(100% - 24px); margin-top: 8px; overflow-y: auto;">
            <template v-if="filteredPrependSeq.length > 0">
              <GroupItem
                v-for="item in filteredPrependSeq"
                :key="item.name"
                type="prepend"
                :group="item"
                @delete="prependSeq = prependSeq.filter(v => v.name !== item.name)"
              />
            </template>
            <GroupItem
              v-for="group in filteredGroupList"
              :key="group.name"
              :type="deleteSeq.includes(group.name) ? 'delete' : 'original'"
              :group="group"
              @delete="deleteSeq = deleteSeq.includes(group.name) ? deleteSeq.filter(v => v !== group.name) : [...deleteSeq, group.name]"
            />
            <GroupItem
              v-for="item in filteredAppendSeq"
              :key="item.name"
              type="append"
              :group="item"
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
import GroupItem from '@/components/profile/group-item.vue'
import { getNetworkInterfaces, readProfileFile, saveProfileFile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { useThemeMode } from '@/services/states'
import type { TranslationKey } from '@/types/generated/i18n-keys'
import type { MonacoEditorInstance } from '@/types/monaco'
import getSystem from '@/utils/get-system'

const props = defineProps<{
  proxiesUid: string
  mergeUid: string
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
const interfaceNameList = ref<string[]>([])

const builtinProxyPolicies = ['DIRECT', 'REJECT', 'REJECT-DROP', 'PASS']
const strategyOptions = ['select', 'url-test', 'fallback', 'load-balance', 'relay']
const excludeTypeOptions = ['Direct', 'Reject', 'RejectDrop', 'Compatible', 'Pass', 'Dns', 'Shadowsocks', 'ShadowsocksR', 'Snell', 'Socks5', 'Http', 'Vmess', 'Vless', 'Trojan', 'Hysteria', 'Hysteria2', 'WireGuard', 'Tuic', 'Mieru', 'Masque', 'AnyTLS', 'Sudoku', 'Relay', 'Selector', 'Fallback', 'URLTest', 'LoadBalance', 'Ssh']

const PROXY_STRATEGY_LABEL_KEYS: Record<string, TranslationKey> = {
  select: 'proxies.components.enums.strategies.select',
  'url-test': 'proxies.components.enums.strategies.url-test',
  fallback: 'proxies.components.enums.strategies.fallback',
  'load-balance': 'proxies.components.enums.strategies.load-balance',
  relay: 'proxies.components.enums.strategies.relay',
}

const PROXY_POLICY_LABEL_KEYS: Record<string, TranslationKey> = builtinProxyPolicies.reduce(
  (acc, policy) => { acc[policy] = `proxies.components.enums.policies.${policy}` as TranslationKey; return acc },
  {} as Record<string, TranslationKey>,
)

const translateStrategy = (value: string) => PROXY_STRATEGY_LABEL_KEYS[value] ? t(PROXY_STRATEGY_LABEL_KEYS[value]) : value
const translatePolicy = (value: string) => PROXY_POLICY_LABEL_KEYS[value] ? t(PROXY_POLICY_LABEL_KEYS[value]) : value

const formType = ref('select')
const formName = ref('')
const formIcon = ref('')
const formProxies = ref<string[]>([])
const formUse = ref<string[]>([])
const formUrl = ref('')
const formExpectedStatus = ref<number>()
const formInterval = ref(300)
const formTimeout = ref(5000)
const formMaxFailedTimes = ref(5)
const formInterfaceName = ref('')
const formRoutingMark = ref<number>()
const formFilter = ref('')
const formExcludeFilter = ref('')
const formExcludeType = ref<string[]>([])
const formIncludeAll = ref(false)
const formIncludeAllProxies = ref(false)
const formIncludeAllProviders = ref(false)
const formLazy = ref(true)
const formDisableUdp = ref(false)
const formHidden = ref(false)

const getFormValues = (): IProxyGroupConfig => ({
  type: formType.value,
  name: formName.value,
  icon: formIcon.value,
  proxies: formProxies.value,
  use: formUse.value,
  url: formUrl.value,
  'expected-status': formExpectedStatus.value,
  interval: formInterval.value,
  timeout: formTimeout.value,
  'max-failed-times': formMaxFailedTimes.value,
  'interface-name': formInterfaceName.value,
  'routing-mark': formRoutingMark.value,
  filter: formFilter.value,
  'exclude-filter': formExcludeFilter.value,
  'exclude-type': formExcludeType.value.join('|'),
  'include-all': formIncludeAll.value,
  'include-all-proxies': formIncludeAllProxies.value,
  'include-all-providers': formIncludeAllProviders.value,
  lazy: formLazy.value,
  'disable-udp': formDisableUdp.value,
  hidden: formHidden.value,
})

const groupList = ref<IProxyGroupConfig[]>([])
const proxyPolicyList = ref<string[]>([])
const proxyProviderList = ref<string[]>([])
const prependSeq = ref<IProxyGroupConfig[]>([])
const appendSeq = ref<IProxyGroupConfig[]>([])
const deleteSeq = ref<string[]>([])

const matches = (name: string) => !matchText.value || name.toLowerCase().includes(matchText.value.toLowerCase())

const filteredPrependSeq = computed(() => prependSeq.value.filter(g => matches(g.name)))
const filteredGroupList = computed(() => groupList.value.filter(g => matches(g.name)))
const filteredAppendSeq = computed(() => appendSeq.value.filter(g => matches(g.name)))

const onSearch = (text: string) => { matchText.value = text }

const normalizeDeleteSeq = (input?: unknown): string[] => {
  if (!Array.isArray(input)) return []
  return Array.from(new Set(input.map((item) => {
    if (typeof item === 'string') return item
    if (item && typeof item === 'object' && 'name' in item && typeof (item as any).name === 'string') return (item as any).name
    return undefined
  }).filter((name): name is string => typeof name === 'string' && name.length > 0)))
}

const buildGroupsYaml = (prepend: IProxyGroupConfig[], append: IProxyGroupConfig[], deleteList: string[]) => {
  return yaml.dump({ prepend, append, delete: deleteList }, { forceQuotes: true })
}

const fetchContent = async () => {
  const data = await readProfileFile(props.property)
  const obj = yaml.load(data) as ISeqProfileConfig | null
  prependSeq.value = obj?.prepend || []
  appendSeq.value = obj?.append || []
  deleteSeq.value = normalizeDeleteSeq(obj?.delete)
  prevData.value = data
  currData.value = data
}

const fetchProxyPolicy = async () => {
  const data = await readProfileFile(props.profileUid)
  const proxiesData = await readProfileFile(props.proxiesUid)
  const originGroupsObj = yaml.load(data) as { 'proxy-groups': IProxyGroupConfig[] } | null
  const originProxiesObj = yaml.load(data) as { proxies: [] } | null
  const originProxies = originProxiesObj?.proxies || []
  const moreProxiesObj = yaml.load(proxiesData) as ISeqProfileConfig | null
  const morePrependProxies = moreProxiesObj?.prepend || []
  const moreAppendProxies = moreProxiesObj?.append || []
  const moreDeleteProxies = normalizeDeleteSeq(moreProxiesObj?.delete)
  const proxies = morePrependProxies.concat(
    originProxies.filter((proxy: any) => {
      const proxyName = typeof proxy === 'string' ? proxy : (proxy?.name as string | undefined)
      return proxyName ? !moreDeleteProxies.includes(proxyName) : true
    }),
    moreAppendProxies,
  )
  const proxyNames = proxies.map((p: any) => typeof p === 'string' ? p : p?.name).filter(Boolean)
  const computedPolicyList = builtinProxyPolicies.concat(
    prependSeq.value.map(g => g.name),
    (originGroupsObj?.['proxy-groups'] || []).map(g => g.name).filter(name => !deleteSeq.value.includes(name)),
    appendSeq.value.map(g => g.name),
    proxyNames,
  )
  proxyPolicyList.value = Array.from(new Set(computedPolicyList))
}

const fetchProfile = async () => {
  const data = await readProfileFile(props.profileUid)
  const mergeData = await readProfileFile(props.mergeUid)
  const globalMergeData = await readProfileFile('Merge')
  const originGroupsObj = yaml.load(data) as { 'proxy-groups': IProxyGroupConfig[] } | null
  const originProviderObj = yaml.load(data) as { 'proxy-providers': Record<string, unknown> } | null
  const originProvider = originProviderObj?.['proxy-providers'] || {}
  const moreProviderObj = yaml.load(mergeData) as { 'proxy-providers': Record<string, unknown> } | null
  const moreProvider = moreProviderObj?.['proxy-providers'] || {}
  const globalProviderObj = yaml.load(globalMergeData) as { 'proxy-providers': Record<string, unknown> } | null
  const globalProvider = globalProviderObj?.['proxy-providers'] || {}
  const provider = Object.assign({}, originProvider, moreProvider, globalProvider)
  proxyProviderList.value = Object.keys(provider)
  groupList.value = originGroupsObj?.['proxy-groups'] || []
}

const getInterfaceNameList = async () => {
  const list = await getNetworkInterfaces()
  interfaceNameList.value = list
}

watch(currData, (val) => {
  if (!val || !visualization.value) return
  const obj = yaml.load(val) as ISeqProfileConfig | null
  prependSeq.value = obj?.prepend ?? []
  appendSeq.value = obj?.append ?? []
  deleteSeq.value = normalizeDeleteSeq(obj?.delete)
})

watch([prependSeq, appendSeq, deleteSeq], () => {
  const serialize = () => {
    try { currData.value = buildGroupsYaml(prependSeq.value, appendSeq.value, deleteSeq.value) }
    catch { /* ignore */ }
  }
  const handle = requestIdleCallback(serialize)
  onUnmounted(() => cancelIdleCallback(handle))
})

watch(() => props.open, (val) => {
  if (val) {
    fetchContent()
    fetchProfile()
    fetchProxyPolicy()
    getInterfaceNameList()
  }
})

const onCloseWrapper = (val: boolean) => { if (!val) props.onClose() }

const validateGroup = () => {
  if (formName.value === '') throw new Error(t('profiles.modals.groupsEditor.errors.nameRequired'))
}

const tryPrepend = () => {
  try {
    validateGroup()
    for (const item of [...prependSeq.value, ...groupList.value]) {
      if (item.name === formName.value) throw new Error(t('profiles.modals.groupsEditor.errors.nameExists'))
    }
    prependSeq.value = [getFormValues(), ...prependSeq.value]
  } catch (err) { showNotice.error(err) }
}

const tryAppend = () => {
  try {
    validateGroup()
    for (const item of [...appendSeq.value, ...groupList.value]) {
      if (item.name === formName.value) throw new Error(t('profiles.modals.groupsEditor.errors.nameExists'))
    }
    appendSeq.value = [...appendSeq.value, getFormValues()]
  } catch (err) { showNotice.error(err) }
}

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
    const nextData = visualization.value ? buildGroupsYaml(prependSeq.value, appendSeq.value, deleteSeq.value) : currData.value
    if (visualization.value) currData.value = nextData
    if (!(await saveProfileFile(props.property, nextData))) { await fetchContent(); props.onClose(); return }
    showNotice.success('shared.feedback.notifications.saved')
    prevData.value = nextData
    props.onSave?.(prevData.value, nextData)
    props.onClose()
  } catch (err) { showNotice.error(err) }
  finally { saveLock = false }
}
</script>
