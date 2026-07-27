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
        <span>{{ t('rules.modals.editor.title') }}</span>
        <n-button size="small" @click="setVisualization(!visualization)">
          {{ visualization ? t('shared.editorModes.advanced') : t('shared.editorModes.visualization') }}
        </n-button>
      </div>
    </template>
    <div style="display: flex; width: auto; height: calc(100vh - 185px);">
      <template v-if="visualization">
        <div style="width: 50%; padding: 0 10px;">
          <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
            <span style="font-size: 14px; font-weight: 500;">{{ t('rules.modals.editor.form.labels.type') }}</span>
            <select
              v-model="ruleTypeIndex"
              style="min-width: 240px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
            >
              <option v-for="(r, i) in rules" :key="r.name" :value="i">
                {{ t(RULE_TYPE_LABEL_KEYS[r.name] ?? r.name) }}
              </option>
            </select>
          </div>
          <div v-if="currentRule.required !== false" style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
            <span style="font-size: 14px; font-weight: 500;">{{ t('rules.modals.editor.form.labels.content') }}</span>
            <template v-if="currentRule.name === 'RULE-SET'">
              <select
                v-model="ruleContent"
                style="min-width: 240px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="rs in ruleSetList" :key="rs" :value="rs">{{ rs }}</option>
              </select>
            </template>
            <template v-else-if="currentRule.name === 'SUB-RULE'">
              <select
                v-model="ruleContent"
                style="min-width: 240px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
              >
                <option v-for="sr in subRuleList" :key="sr" :value="sr">{{ sr }}</option>
              </select>
            </template>
            <template v-else>
              <input
                v-model="ruleContent"
                :placeholder="currentRule.example"
                style="min-width: 240px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
                :class="{ error: (currentRule.required ?? true) && !ruleContent }"
              />
            </template>
          </div>
          <div style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
            <span style="font-size: 14px; font-weight: 500;">{{ t('rules.modals.editor.form.labels.proxyPolicy') }}</span>
            <select
              v-model="proxyPolicy"
              style="min-width: 240px; padding: 4px 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
            >
              <option v-for="pp in proxyPolicyList" :key="pp" :value="pp">
                {{ t(PROXY_POLICY_LABEL_KEYS[pp] ?? pp) }}
              </option>
            </select>
          </div>
          <div v-if="currentRule.noResolve" style="padding: 5px 2px; display: flex; align-items: center; justify-content: space-between;">
            <span style="font-size: 14px; font-weight: 500;">{{ t('rules.modals.editor.form.toggles.noResolve') }}</span>
            <label><input type="checkbox" v-model="noResolve" /></label>
          </div>
          <div style="padding: 5px 2px;">
            <n-button block @click="prependRule">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M8 11h3v10h2V11h3l-4-4-4 4zM4 3v2h16V3H4z"/></svg>
              </template>
              {{ t('rules.modals.editor.form.actions.prependRule') }}
            </n-button>
          </div>
          <div style="padding: 5px 2px;">
            <n-button block @click="appendRule">
              <template #icon>
                <svg viewBox="0 0 24 24" width="16" height="16" fill="currentColor"><path d="M16 13h-3V3h-2v10H8l4 4 4-4zM4 19v2h16v-2H4z"/></svg>
              </template>
              {{ t('rules.modals.editor.form.actions.appendRule') }}
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
              <RuleItem
                v-for="item in filteredPrependSeq"
                :key="item"
                type="prepend"
                :rule-raw="item"
                @delete="prependSeq = prependSeq.filter(v => v !== item)"
              />
            </template>
            <RuleItem
              v-for="rule in filteredRuleList"
              :key="rule"
              :type="deleteSeq.includes(rule) ? 'delete' : 'original'"
              :rule-raw="rule"
              @delete="deleteSeq = deleteSeq.includes(rule) ? deleteSeq.filter(v => v !== rule) : [...deleteSeq, rule]"
            />
            <RuleItem
              v-for="item in filteredAppendSeq"
              :key="item"
              type="append"
              :rule-raw="item"
              @delete="appendSeq = appendSeq.filter(v => v !== item)"
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
import RuleItem from '@/components/profile/rule-item.vue'
import { readProfileFile, saveProfileFile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { useThemeMode } from '@/services/states'
import type { TranslationKey } from '@/types/generated/i18n-keys'
import type { MonacoEditorInstance } from '@/types/monaco'
import getSystem from '@/utils/get-system'
import { isValidIpCidr } from '@/utils/network'

const props = defineProps<{
  groupsUid: string
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

const builtinProxyPolicies = ['DIRECT', 'REJECT', 'REJECT-DROP', 'PASS']

const portValidator = (value: string): boolean => {
  return new RegExp('^(?:[1-9]\\d{0,3}|[1-5]\\d{4}|6[0-4]\\d{3}|65[0-4]\\d{2}|655[0-2]\\d|6553[0-5])$').test(value)
}

const rules = [
  { name: 'DOMAIN', example: 'example.com' },
  { name: 'DOMAIN-SUFFIX', example: 'example.com' },
  { name: 'DOMAIN-KEYWORD', example: 'example' },
  { name: 'DOMAIN-REGEX', example: 'example.*' },
  { name: 'GEOSITE', example: 'youtube' },
  { name: 'GEOIP', example: 'CN', noResolve: true },
  { name: 'SRC-GEOIP', example: 'CN' },
  { name: 'IP-ASN', example: '13335', noResolve: true, validator: (v: string) => !!+v },
  { name: 'SRC-IP-ASN', example: '9808', validator: (v: string) => !!+v },
  { name: 'IP-CIDR', example: '127.0.0.0/8', noResolve: true, validator: isValidIpCidr },
  { name: 'IP-CIDR6', example: '2620:0:2d0:200::7/32', noResolve: true, validator: isValidIpCidr },
  { name: 'SRC-IP-CIDR', example: '192.168.1.201/32', validator: isValidIpCidr },
  { name: 'IP-SUFFIX', example: '8.8.8.8/24', noResolve: true, validator: isValidIpCidr },
  { name: 'SRC-IP-SUFFIX', example: '192.168.1.201/8', validator: isValidIpCidr },
  { name: 'SRC-PORT', example: '7777', validator: portValidator },
  { name: 'DST-PORT', example: '80', validator: portValidator },
  { name: 'IN-PORT', example: '7897', validator: portValidator },
  { name: 'DSCP', example: '4' },
  { name: 'PROCESS-NAME', example: getSystem() === 'windows' ? 'chrome.exe' : 'curl' },
  { name: 'PROCESS-PATH', example: getSystem() === 'windows' ? 'C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe' : '/usr/bin/wget' },
  { name: 'PROCESS-NAME-REGEX', example: '.*telegram.*' },
  { name: 'PROCESS-PATH-REGEX', example: getSystem() === 'windows' ? '(?i).*Application\\chrome.*' : '.*bin/wget' },
  { name: 'NETWORK', example: 'udp', validator: (v: string) => ['tcp', 'udp'].includes(v) },
  { name: 'UID', example: '1001', validator: (v: string) => !!+v },
  { name: 'IN-TYPE', example: 'SOCKS/HTTP' },
  { name: 'IN-USER', example: 'mihomo' },
  { name: 'IN-NAME', example: 'ss' },
  { name: 'SUB-RULE', example: '(NETWORK,tcp)' },
  { name: 'RULE-SET', example: 'providername', noResolve: true },
  { name: 'AND', example: '((DOMAIN,baidu.com),(NETWORK,UDP))' },
  { name: 'OR', example: '((NETWORK,UDP),(DOMAIN,baidu.com))' },
  { name: 'NOT', example: '((DOMAIN,baidu.com))' },
  { name: 'MATCH', required: false },
]

const RULE_TYPE_LABEL_KEYS: Record<string, string> = Object.fromEntries(
  rules.map(r => [r.name, `rules.modals.editor.ruleTypes.${r.name}`]),
)

const PROXY_POLICY_LABEL_KEYS: Record<string, TranslationKey> = builtinProxyPolicies.reduce(
  (acc, policy) => { acc[policy] = `proxies.components.enums.policies.${policy}` as TranslationKey; return acc },
  {} as Record<string, TranslationKey>,
)

const ruleTypeIndex = ref(0)
const ruleContent = ref('')
const noResolve = ref(false)
const proxyPolicy = ref(builtinProxyPolicies[0])
const proxyPolicyList = ref<string[]>([])
const ruleList = ref<string[]>([])
const ruleSetList = ref<string[]>([])
const subRuleList = ref<string[]>([])
const prependSeq = ref<string[]>([])
const appendSeq = ref<string[]>([])
const deleteSeq = ref<string[]>([])

const currentRule = computed(() => rules[ruleTypeIndex.value])

const matches = (rule: string) => !matchText.value || rule.toLowerCase().includes(matchText.value.toLowerCase())

const filteredPrependSeq = computed(() => prependSeq.value.filter(r => matches(r)))
const filteredRuleList = computed(() => ruleList.value.filter(r => matches(r)))
const filteredAppendSeq = computed(() => appendSeq.value.filter(r => matches(r)))

const onSearch = (text: string) => { matchText.value = text }

const fetchContent = async () => {
  const data = await readProfileFile(props.property)
  const obj = yaml.load(data) as ISeqProfileConfig | null
  prependSeq.value = obj?.prepend || []
  appendSeq.value = obj?.append || []
  deleteSeq.value = obj?.delete || []
  prevData.value = data
  currData.value = data
}

watch(currData, (val) => {
  if (!val || !visualization.value) return
  const obj = yaml.load(val) as ISeqProfileConfig | null
  prependSeq.value = obj?.prepend ?? []
  appendSeq.value = obj?.append ?? []
  deleteSeq.value = obj?.delete ?? []
})

let hasLoadedContent = false
watch([prependSeq, appendSeq, deleteSeq], () => {
  if (!hasLoadedContent) return
  const serialize = () => {
    try {
      currData.value = yaml.dump({ prepend: prependSeq.value, append: appendSeq.value, delete: deleteSeq.value }, { forceQuotes: true })
    } catch { /* ignore */ }
  }
  let idleId: number | undefined
  let timeoutId: number | undefined
  if (window.requestIdleCallback) { idleId = window.requestIdleCallback(serialize) }
  else { timeoutId = window.setTimeout(serialize, 0) }
  onUnmounted(() => {
    if (idleId !== undefined && window.cancelIdleCallback) window.cancelIdleCallback(idleId)
    if (timeoutId !== undefined) clearTimeout(timeoutId)
  })
})

const fetchProfile = async () => {
  const data = await readProfileFile(props.profileUid)
  const groupsData = await readProfileFile(props.groupsUid)
  const mergeData = await readProfileFile(props.mergeUid)
  const globalMergeData = await readProfileFile('Merge')

  const rulesObj = yaml.load(data) as { rules: [] } | null
  const originGroupsObj = yaml.load(data) as { 'proxy-groups': IProxyGroupConfig[] } | null
  const originGroups = originGroupsObj?.['proxy-groups'] || []

  const moreGroupsObj = yaml.load(groupsData) as ISeqProfileConfig | null
  const morePrependGroups = Array.isArray(moreGroupsObj?.['prepend']) ? moreGroupsObj['prepend'] as IProxyGroupConfig[] : []
  const moreAppendGroups = Array.isArray(moreGroupsObj?.['append']) ? moreGroupsObj['append'] as IProxyGroupConfig[] : []
  const rawDeleteGroups = moreGroupsObj?.['delete']
  const moreDeleteGroups: Array<string | { name: string }> = Array.isArray(rawDeleteGroups) ? rawDeleteGroups as Array<string | { name: string }> : []

  const groups = morePrependGroups.concat(
    originGroups.filter((g: any) => !g.name || !moreDeleteGroups.includes(g.name)),
    moreAppendGroups,
  )

  const originRuleSetObj = yaml.load(data) as { 'rule-providers': Record<string, unknown> } | null
  const originRuleSet = originRuleSetObj?.['rule-providers'] || {}
  const moreRuleSetObj = yaml.load(mergeData) as { 'rule-providers': Record<string, unknown> } | null
  const moreRuleSet = moreRuleSetObj?.['rule-providers'] || {}
  const globalRuleSetObj = yaml.load(globalMergeData) as { 'rule-providers': Record<string, unknown> } | null
  const globalRuleSet = globalRuleSetObj?.['rule-providers'] || {}
  const ruleSet = Object.assign({}, originRuleSet, moreRuleSet, globalRuleSet)

  const originSubRuleObj = yaml.load(data) as { 'sub-rules': Record<string, unknown> } | null
  const originSubRule = originSubRuleObj?.['sub-rules'] || {}
  const moreSubRuleObj = yaml.load(mergeData) as { 'sub-rules': Record<string, unknown> } | null
  const moreSubRule = moreSubRuleObj?.['sub-rules'] || {}
  const globalSubRuleObj = yaml.load(globalMergeData) as { 'sub-rules': Record<string, unknown> } | null
  const globalSubRule = globalSubRuleObj?.['sub-rules'] || {}
  const subRule = Object.assign({}, originSubRule, moreSubRule, globalSubRule)

  proxyPolicyList.value = builtinProxyPolicies.concat(groups.map((g: any) => g.name))
  ruleSetList.value = Object.keys(ruleSet)
  subRuleList.value = Object.keys(subRule)
  ruleList.value = rulesObj?.rules || []
  hasLoadedContent = true
}

watch(() => props.open, (val) => {
  if (val) { fetchContent(); fetchProfile() }
})

const onCloseWrapper = (val: boolean) => { if (!val) props.onClose() }

const validateRule = () => {
  const ruleType = currentRule.value
  if ((ruleType.required ?? true) && !ruleContent.value) {
    throw new Error(t('rules.modals.editor.form.validation.conditionRequired'))
  }
  if (ruleType.validator && !ruleType.validator(ruleContent.value)) {
    throw new Error(t('rules.modals.editor.form.validation.invalidRule'))
  }
  const condition = (ruleType.required ?? true) ? ruleContent.value : ''
  return `${ruleType.name}${condition ? ',' + condition : ''},${proxyPolicy.value}${ruleType.noResolve && noResolve.value ? ',no-resolve' : ''}`
}

const prependRule = () => {
  try {
    const raw = validateRule()
    if (prependSeq.value.includes(raw)) return
    prependSeq.value = [raw, ...prependSeq.value]
  } catch (err) { showNotice.error(err) }
}

const appendRule = () => {
  try {
    const raw = validateRule()
    if (appendSeq.value.includes(raw)) return
    appendSeq.value = [...appendSeq.value, raw]
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
    if (!(await saveProfileFile(props.property, currData.value))) { await fetchContent(); props.onClose(); return }
    showNotice.success('shared.feedback.notifications.saved')
    props.onSave?.(prevData.value, currData.value)
    props.onClose()
  } catch (err) { showNotice.error(err) }
  finally { saveLock = false }
}
</script>
