<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted, nextTick } from 'vue'
import i18n from 'i18next'
import { invoke } from '@tauri-apps/api/core'
import yaml from 'js-yaml'

import { BaseDialog, DialogRef, MonacoEditor, Switch } from '@/components/base'
import { useClash } from '@/hooks/use-clash'
import { showNotice } from '@/services/notice-service'
import { useThemeMode } from '@/services/states'
import type { MonacoEditorInstance } from '@/types/monaco'
import getSystem from '@/utils/get-system'

const DEFAULT_DNS_CONFIG = {
  enable: true,
  listen: ':53',
  'enhanced-mode': 'fake-ip' as const,
  'fake-ip-range': '198.18.0.1/16',
  'fake-ip-range6': 'fdfe:dcba:9876::1/64',
  'fake-ip-filter-mode': 'blacklist' as const,
  'prefer-h3': false,
  'respect-rules': false,
  'use-hosts': false,
  'use-system-hosts': false,
  ipv6: true,
  'fake-ip-filter': ['*.lan', '*.local', '*.arpa', 'time.*.com', 'ntp.*.com',
    '+.market.xiaomi.com', 'localhost.ptlogin2.qq.com', '*.msftncsi.com', 'www.msftconnecttest.com'],
  'default-nameserver': ['system', '223.6.6.6', '8.8.8.8', '2400:3200::1', '2001:4860:4860::8888'],
  nameserver: ['8.8.8.8', 'https://doh.pub/dns-query', 'https://dns.alidns.com/dns-query'],
  fallback: [],
  'nameserver-policy': {},
  'proxy-server-nameserver': ['https://doh.pub/dns-query', 'https://dns.alidns.com/dns-query', 'tls://223.5.5.5'],
  'direct-nameserver': [],
  'direct-nameserver-follow-policy': false,
  'fallback-filter': { geoip: true, 'geoip-code': 'CN', ipcidr: ['240.0.0.0/4', '0.0.0.0/32'], domain: ['+.google.com', '+.facebook.com', '+.youtube.com'] },
}

function parseList(str: string): string[] {
  if (!str?.trim()) return []
  return str.split(',').map((item) => item.trim()).filter(Boolean)
}

function formatNameserverPolicy(policy: unknown): string {
  if (!policy || typeof policy !== 'object') return ''
  return Object.entries(policy as Record<string, unknown>)
    .map(([domain, servers]) => {
      const serversStr = Array.isArray(servers) ? servers.join(';') : servers
      return `${domain}=${serversStr}`
    })
    .join(', ')
}

function parseNameserverPolicy(str: string): Record<string, any> {
  const result: Record<string, any> = {}
  if (!str) return result
  const ruleRegex = /\s*([^=]+?)\s*=\s*([^,]+)(?:,|$)/g
  let match: RegExpExecArray | null
  while ((match = ruleRegex.exec(str)) !== null) {
    const [, domainsPart, serversPart] = match
    const domains = [domainsPart.trim()]
    const servers = serversPart.split(';').map((s) => s.trim())
    domains.forEach((domain) => { result[domain] = servers })
  }
  return result
}

function formatHosts(hosts: unknown): string {
  if (!hosts || typeof hosts !== 'object') return ''
  const result: string[] = []
  Object.entries(hosts as Record<string, unknown>).forEach(([domain, value]) => {
    if (Array.isArray(value)) {
      result.push(`${domain}=${value.join(';')}`)
    } else {
      result.push(`${domain}=${value}`)
    }
  })
  return result.join(', ')
}

function parseHosts(str: string): Record<string, any> {
  const result: Record<string, any> = {}
  if (!str) return result
  str.split(',').forEach((item) => {
    const parts = item.trim().split('=')
    if (parts.length < 2) return
    const domain = parts[0].trim()
    const valueStr = parts.slice(1).join('=').trim()
    if (valueStr.includes(';')) {
      result[domain] = valueStr.split(';').map((s) => s.trim()).filter(Boolean)
    } else {
      result[domain] = valueStr
    }
  })
  return result
}

const { clash, mutateClash } = useClash()
const themeMode = useThemeMode()

const open = ref(false)
const visualization = ref(true)
const skipYamlSyncRef = ref(false)
const editorRef = ref<MonacoEditorInstance | null>(null)

const defaultValues = {
  enable: DEFAULT_DNS_CONFIG.enable,
  listen: DEFAULT_DNS_CONFIG.listen,
  enhancedMode: DEFAULT_DNS_CONFIG['enhanced-mode'],
  fakeIpRange: DEFAULT_DNS_CONFIG['fake-ip-range'],
  fakeIpRange6: DEFAULT_DNS_CONFIG['fake-ip-range6'],
  fakeIpFilterMode: DEFAULT_DNS_CONFIG['fake-ip-filter-mode'],
  preferH3: DEFAULT_DNS_CONFIG['prefer-h3'],
  respectRules: DEFAULT_DNS_CONFIG['respect-rules'],
  useHosts: DEFAULT_DNS_CONFIG['use-hosts'],
  useSystemHosts: DEFAULT_DNS_CONFIG['use-system-hosts'],
  ipv6: DEFAULT_DNS_CONFIG.ipv6,
  fakeIpFilter: DEFAULT_DNS_CONFIG['fake-ip-filter'].join(', '),
  defaultNameserver: DEFAULT_DNS_CONFIG['default-nameserver'].join(', '),
  nameserver: DEFAULT_DNS_CONFIG.nameserver.join(', '),
  fallback: DEFAULT_DNS_CONFIG.fallback.join(', '),
  proxyServerNameserver: DEFAULT_DNS_CONFIG['proxy-server-nameserver']?.join(', ') || '',
  directNameserver: DEFAULT_DNS_CONFIG['direct-nameserver']?.join(', ') || '',
  directNameserverFollowPolicy: DEFAULT_DNS_CONFIG['direct-nameserver-follow-policy'] || false,
  fallbackGeoip: DEFAULT_DNS_CONFIG['fallback-filter'].geoip,
  fallbackGeoipCode: DEFAULT_DNS_CONFIG['fallback-filter']['geoip-code'],
  fallbackIpcidr: DEFAULT_DNS_CONFIG['fallback-filter'].ipcidr?.join(', ') || '',
  fallbackDomain: DEFAULT_DNS_CONFIG['fallback-filter'].domain?.join(', ') || '',
  nameserverPolicy: '',
  hosts: '',
}

const values = ref({ ...defaultValues })
const yamlContent = ref('')

function updateValuesFromConfig(config: any) {
  if (!config) return
  const dnsConfig = config.dns || {}
  const hostsConfig = config.hosts || {}
  const enhancedMode = dnsConfig['enhanced-mode'] || DEFAULT_DNS_CONFIG['enhanced-mode']
  const validEnhancedMode = enhancedMode === 'fake-ip' || enhancedMode === 'redir-host' ? enhancedMode : DEFAULT_DNS_CONFIG['enhanced-mode']
  const fakeIpFilterMode = dnsConfig['fake-ip-filter-mode'] || DEFAULT_DNS_CONFIG['fake-ip-filter-mode']
  const validFakeIpFilterMode = fakeIpFilterMode === 'blacklist' || fakeIpFilterMode === 'whitelist' ? fakeIpFilterMode : DEFAULT_DNS_CONFIG['fake-ip-filter-mode']

  values.value = {
    enable: dnsConfig.enable ?? DEFAULT_DNS_CONFIG.enable,
    listen: dnsConfig.listen ?? DEFAULT_DNS_CONFIG.listen,
    enhancedMode: validEnhancedMode,
    fakeIpRange: dnsConfig['fake-ip-range'] ?? DEFAULT_DNS_CONFIG['fake-ip-range'],
    fakeIpRange6: dnsConfig['fake-ip-range6'] ?? DEFAULT_DNS_CONFIG['fake-ip-range6'],
    fakeIpFilterMode: validFakeIpFilterMode,
    preferH3: dnsConfig['prefer-h3'] ?? DEFAULT_DNS_CONFIG['prefer-h3'],
    respectRules: dnsConfig['respect-rules'] ?? DEFAULT_DNS_CONFIG['respect-rules'],
    useHosts: dnsConfig['use-hosts'] ?? DEFAULT_DNS_CONFIG['use-hosts'],
    useSystemHosts: dnsConfig['use-system-hosts'] ?? DEFAULT_DNS_CONFIG['use-system-hosts'],
    ipv6: dnsConfig.ipv6 ?? DEFAULT_DNS_CONFIG.ipv6,
    fakeIpFilter: dnsConfig['fake-ip-filter']?.join(', ') ?? DEFAULT_DNS_CONFIG['fake-ip-filter'].join(', '),
    nameserver: dnsConfig.nameserver?.join(', ') ?? DEFAULT_DNS_CONFIG.nameserver.join(', '),
    fallback: dnsConfig.fallback?.join(', ') ?? DEFAULT_DNS_CONFIG.fallback.join(', '),
    defaultNameserver: dnsConfig['default-nameserver']?.join(', ') ?? DEFAULT_DNS_CONFIG['default-nameserver'].join(', '),
    proxyServerNameserver: dnsConfig['proxy-server-nameserver']?.join(', ') ?? (DEFAULT_DNS_CONFIG['proxy-server-nameserver']?.join(', ') || ''),
    directNameserver: dnsConfig['direct-nameserver']?.join(', ') ?? (DEFAULT_DNS_CONFIG['direct-nameserver']?.join(', ') || ''),
    directNameserverFollowPolicy: dnsConfig['direct-nameserver-follow-policy'] ?? DEFAULT_DNS_CONFIG['direct-nameserver-follow-policy'],
    fallbackGeoip: dnsConfig['fallback-filter']?.geoip ?? DEFAULT_DNS_CONFIG['fallback-filter'].geoip,
    fallbackGeoipCode: dnsConfig['fallback-filter']?.['geoip-code'] ?? DEFAULT_DNS_CONFIG['fallback-filter']['geoip-code'],
    fallbackIpcidr: dnsConfig['fallback-filter']?.ipcidr?.join(', ') ?? DEFAULT_DNS_CONFIG['fallback-filter'].ipcidr.join(', '),
    fallbackDomain: dnsConfig['fallback-filter']?.domain?.join(', ') ?? DEFAULT_DNS_CONFIG['fallback-filter'].domain.join(', '),
    nameserverPolicy: formatNameserverPolicy(dnsConfig['nameserver-policy']) || '',
    hosts: formatHosts(hostsConfig) || '',
  }
}

function generateDnsConfig() {
  const v = values.value
  const dnsConfig: any = {
    enable: v.enable,
    listen: v.listen,
    'enhanced-mode': v.enhancedMode,
    'fake-ip-range': v.fakeIpRange,
    'fake-ip-range6': v.fakeIpRange6 || DEFAULT_DNS_CONFIG['fake-ip-range6'],
    'fake-ip-filter-mode': v.fakeIpFilterMode,
    'prefer-h3': v.preferH3,
    'respect-rules': v.respectRules,
    'use-hosts': v.useHosts,
    'use-system-hosts': v.useSystemHosts,
    ipv6: v.ipv6,
    'fake-ip-filter': parseList(v.fakeIpFilter),
    'default-nameserver': parseList(v.defaultNameserver),
    nameserver: parseList(v.nameserver),
    'direct-nameserver-follow-policy': v.directNameserverFollowPolicy,
    'fallback-filter': {
      geoip: v.fallbackGeoip,
      'geoip-code': v.fallbackGeoipCode,
      ipcidr: parseList(v.fallbackIpcidr),
      domain: parseList(v.fallbackDomain),
    },
    fallback: parseList(v.fallback),
    'proxy-server-nameserver': parseList(v.proxyServerNameserver),
    'direct-nameserver': parseList(v.directNameserver),
  }
  const policy = parseNameserverPolicy(v.nameserverPolicy)
  if (Object.keys(policy).length > 0) {
    dnsConfig['nameserver-policy'] = policy
  }
  return dnsConfig
}

function updateYamlFromValues() {
  const config: Record<string, any> = {}
  const dnsConfig = generateDnsConfig()
  if (Object.keys(dnsConfig).length > 0) config.dns = dnsConfig
  const hosts = parseHosts(values.value.hosts)
  if (Object.keys(hosts).length > 0) config.hosts = hosts
  yamlContent.value = yaml.dump(config, { forceQuotes: true })
}

function updateValuesFromYaml() {
  try {
    const parsedYaml = yaml.load(yamlContent.value) as any
    if (!parsedYaml) return
    skipYamlSyncRef.value = true
    updateValuesFromConfig(parsedYaml)
  } catch {
    showNotice.error('settings.modals.dns.errors.invalidYaml')
  }
}

watch(yamlContent, () => {
  if (skipYamlSyncRef.value) {
    skipYamlSyncRef.value = false
    return
  }
  updateYamlFromValues()
})

watch(visualization, (val) => {
  if (val) updateValuesFromYaml()
  else updateYamlFromValues()
})

function resetToDefaults() {
  values.value = { ...defaultValues }
  nextTick(() => updateYamlFromValues())
}

let latestUpdateValuesFromYaml: (() => void) | null = null
let latestUpdateYamlFromValues: (() => void) | null = null

watch([updateValuesFromYaml, updateYamlFromValues], ([a, b]) => {
  latestUpdateValuesFromYaml = a
  latestUpdateYamlFromValues = b
})

async function initDnsConfig() {
  try {
    const dnsConfigExists = await invoke<boolean>('check_dns_config_exists', {})
    if (dnsConfigExists) {
      const dnsConfig = await invoke<string>('get_dns_config_content', {})
      const config = yaml.load(dnsConfig) as any
      updateValuesFromConfig(config)
      yamlContent.value = dnsConfig
    } else {
      resetToDefaults()
    }
  } catch (err) {
    console.error('Failed to initialize DNS config', err)
    resetToDefaults()
  }
}

const onSave = async () => {
  try {
    let config: Record<string, any>
    if (visualization.value) {
      config = {}
      const dnsConfig = generateDnsConfig()
      if (Object.keys(dnsConfig).length > 0) config.dns = dnsConfig
      const hosts = parseHosts(values.value.hosts)
      if (Object.keys(hosts).length > 0) config.hosts = hosts
    } else {
      const parsedConfig = yaml.load(yamlContent.value)
      if (typeof parsedConfig !== 'object' || parsedConfig === null) {
        throw new Error(i18n.t('settings.modals.dns.errors.invalid'))
      }
      config = parsedConfig as Record<string, any>
    }

    await invoke('save_dns_config', { dnsConfig: config })
    const validation = await invoke<ValidationOutcome>('validate_dns_config', {})
    if (validation.status !== 'valid') {
      const errorMsg = validation.status === 'invalid' ? validation.message : 'Configuration validation skipped'
      let cleanErrorMsg = errorMsg
      if (errorMsg.includes('level=error')) {
        const errorLines = errorMsg.split('\n').filter((line: string) =>
          line.includes('level=error') || line.includes('level=fatal') || line.includes('failed'))
        if (errorLines.length > 0) {
          cleanErrorMsg = errorLines.map((line: string) => {
            const msgMatch = line.match(/msg="([^"]+)"/)
            return msgMatch ? msgMatch[1] : line
          }).join(', ')
        }
      }
      showNotice.error('settings.modals.dns.messages.configError', cleanErrorMsg)
      return
    }
    if (clash?.dns?.enable) {
      await invoke('apply_dns_config', { apply: true })
      mutateClash()
    }
    open.value = false
    showNotice.success('settings.modals.dns.messages.saved')
  } catch (err) {
    showNotice.error('settings.modals.dns.messages.configError', err)
  }
}

const handleYamlChange = (value?: string) => {
  yamlContent.value = value || ''
}

const handleChange = (field: string) => (event: any) => {
  const value = event.target.type === 'checkbox' ? event.target.checked : event.target.value
  values.value = { ...values.value, [field]: value }
  if (visualization.value) {
    nextTick(() => updateYamlFromValues())
  }
}
</script>

<template>
  <BaseDialog
    :open="open"
    :disableEnforceFocus="!visualization"
    :contentSx="{
      width: '550px',
      overflow: 'auto',
      ...(visualization ? {} : { padding: '0 24px', display: 'flex', flexDirection: 'column' })
    }"
    :okBtn="i18n.t('shared.actions.save')"
    :cancelBtn="i18n.t('shared.actions.cancel')"
    @onClose="open = false"
    @onCancel="open = false"
    @onOk="onSave"
  >
    <template #title>
      <div style="display: flex; justify-content: space-between; align-items: center;">
        <span>{{ i18n.t('settings.modals.dns.dialog.title') }}</span>
        <div style="display: flex; align-items: center; gap: 8px;">
          <button class="MuiButton-root MuiButton-outlined MuiButton-sizeSmall" style="color: warning;" @click="resetToDefaults">
            <svg viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor" style="margin-right: 4px;"><path d="M12 5V1L7 6l5 5V7c3.31 0 6 2.69 6 6s-2.69 6-6 6-6-2.69-6-6H4c0 4.42 3.58 8 8 8s8-3.58 8-8-3.58-8-8-8z"/></svg>
            {{ i18n.t('shared.actions.resetToDefault') }}
          </button>
          <button class="MuiButton-root MuiButton-contained MuiButton-sizeSmall" @click="visualization = !visualization">
            {{ visualization ? i18n.t('shared.editorModes.advanced') : i18n.t('shared.editorModes.visualization') }}
          </button>
        </div>
      </div>
    </template>

    <span class="MuiTypography-root MuiTypography-body2" style="color: warning; margin-bottom: 16px; font-style: italic;">
      {{ i18n.t('settings.modals.dns.dialog.warning') }}
    </span>

    <template v-if="visualization">
      <ul class="MuiList-root" style="list-style: none; padding: 0;">
        <!-- General section -->
        <span class="MuiTypography-root MuiTypography-subtitle1" style="margin: 8px 0; font-weight: bold;">{{ i18n.t('settings.modals.dns.sections.general') }}</span>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.enable') }}</span>
          </div>
          <Switch edge="end" :checked="values.enable" @change="handleChange('enable')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.listen') }}</span>
          </div>
          <input :value="values.listen" @input="handleChange('listen')($event)" placeholder=":53" style="width: 150px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.enhancedMode') }}</span>
          </div>
          <select :value="values.enhancedMode" @change="handleChange('enhancedMode')" style="width: 150px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;">
            <option value="fake-ip">fake-ip</option>
            <option value="redir-host">redir-host</option>
          </select>
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.fakeIpRange') }}</span>
          </div>
          <input :value="values.fakeIpRange" @input="handleChange('fakeIpRange')($event)" placeholder="198.18.0.1/16" style="width: 150px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.fakeIpRange6') }}</span>
          </div>
          <input :value="values.fakeIpRange6" @input="handleChange('fakeIpRange6')($event)" placeholder="fdfe:dcba:9876::1/64" style="width: 200px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.fakeIpFilterMode') }}</span>
          </div>
          <select :value="values.fakeIpFilterMode" @change="handleChange('fakeIpFilterMode')" style="width: 150px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;">
            <option value="blacklist">blacklist</option>
            <option value="whitelist">whitelist</option>
          </select>
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.ipv6.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.ipv6" @change="handleChange('ipv6')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.preferH3.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.preferH3" @change="handleChange('preferH3')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.respectRules.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.respectRules" @change="handleChange('respectRules')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.useHosts.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.useHosts" @change="handleChange('useHosts')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.useSystemHosts.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.useSystemHosts" @change="handleChange('useSystemHosts')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.directPolicy.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.directNameserverFollowPolicy" @change="handleChange('directNameserverFollowPolicy')" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.defaultNameserver.label') }}</span>
          </div>
          <textarea :value="values.defaultNameserver" @input="handleChange('defaultNameserver')($event)" placeholder="system,223.6.6.6, 8.8.8.8" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.nameserver.label') }}</span>
          </div>
          <textarea :value="values.nameserver" @input="handleChange('nameserver')($event)" placeholder="8.8.8.8, https://doh.pub/dns-query" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.fallback.label') }}</span>
          </div>
          <textarea :value="values.fallback" @input="handleChange('fallback')($event)" placeholder="https://dns.alidns.com/dns-query" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.proxy.label') }}</span>
          </div>
          <textarea :value="values.proxyServerNameserver" @input="handleChange('proxyServerNameserver')($event)" placeholder="https://doh.pub/dns-query" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.directNameserver.label') }}</span>
          </div>
          <textarea :value="values.directNameserver" @input="handleChange('directNameserver')($event)" placeholder="system, 223.6.6.6" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.fakeIpFilter.label') }}</span>
          </div>
          <textarea :value="values.fakeIpFilter" @input="handleChange('fakeIpFilter')($event)" placeholder="*.lan, *.local" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.nameserverPolicy.label') }}</span>
          </div>
          <textarea :value="values.nameserverPolicy" @input="handleChange('nameserverPolicy')($event)" placeholder="+.arpa=10.0.0.1" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <span class="MuiTypography-root MuiTypography-subtitle2" style="margin: 16px 0 8px; font-weight: bold;">{{ i18n.t('settings.modals.dns.sections.fallbackFilter') }}</span>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.geoipFiltering.label') }}</span>
          </div>
          <Switch edge="end" :checked="values.fallbackGeoip" @change="handleChange('fallbackGeoip')" />
        </li>

        <li style="padding: 5px 2px; display: flex; align-items: center;">
          <div class="MuiListItemText-root" style="flex: 1;">
            <span>{{ i18n.t('settings.modals.dns.fields.geoipCode') }}</span>
          </div>
          <input :value="values.fallbackGeoipCode" @input="handleChange('fallbackGeoipCode')($event)" placeholder="CN" style="width: 100px; padding: 8px; border: 1px solid #ccc; border-radius: 4px;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.fallbackIpCidr.label') }}</span>
          </div>
          <textarea :value="values.fallbackIpcidr" @input="handleChange('fallbackIpcidr')($event)" placeholder="240.0.0.0/4" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.fallbackDomain.label') }}</span>
          </div>
          <textarea :value="values.fallbackDomain" @input="handleChange('fallbackDomain')($event)" placeholder="+.google.com" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>

        <span class="MuiTypography-root MuiTypography-subtitle1" style="margin: 24px 0 0; font-weight: bold;">{{ i18n.t('settings.modals.dns.sections.hosts') }}</span>

        <li style="padding: 5px 2px; display: flex; flex-direction: column; align-items: flex-start;">
          <div class="MuiListItemText-root">
            <span>{{ i18n.t('settings.modals.dns.fields.hosts.label') }}</span>
          </div>
          <textarea :value="values.hosts" @input="handleChange('hosts')($event)" placeholder="*.clash.dev=127.0.0.1" style="width: 100%; min-height: 40px; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;" />
        </li>
      </ul>
    </template>

    <MonacoEditor
      v-else
      height="100vh"
      language="yaml"
      :value="yamlContent"
      :theme="themeMode === 'light' ? 'light' : 'vs-dark'"
      class="flex-grow"
      @onChange="handleYamlChange"
    />
  </BaseDialog>
</template>
