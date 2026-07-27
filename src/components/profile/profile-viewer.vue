<template>
  <BaseDialog
    :open="open"
    :title="openType === 'new' ? t('profiles.modals.profileForm.title.create') : t('profiles.modals.profileForm.title.edit')"
    :content-sx="{ width: 375, pb: 0, maxHeight: '80%' }"
    :ok-btn="t('shared.actions.save')"
    :cancel-btn="t('shared.actions.cancel')"
    :loading="loading"
    @close="handleClose"
    @cancel="handleClose"
    @ok="handleOk"
  >
    <div style="margin-top: 8px; margin-bottom: 8px;">
      <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('profiles.modals.profileForm.fields.type') }}</label>
      <select
        v-model="formType"
        style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
        autofocus
      >
        <option value="remote">{{ t('profiles.modals.profileForm.types.remote') }}</option>
        <option value="local">{{ t('profiles.modals.profileForm.types.local') }}</option>
      </select>
    </div>

    <div style="margin-top: 8px; margin-bottom: 8px;">
      <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('shared.labels.name') }}</label>
      <input
        v-model="formName"
        style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); box-sizing: border-box;"
      />
    </div>

    <div style="margin-top: 8px; margin-bottom: 8px;">
      <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('profiles.modals.profileForm.fields.description') }}</label>
      <input
        v-model="formDesc"
        style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); box-sizing: border-box;"
      />
    </div>

    <FileInput
      v-if="isLocal && openType === 'new'"
      @change="onFileInput"
    />

    <template v-if="isRemote">
      <div style="margin-top: 8px; margin-bottom: 8px;">
        <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('profiles.modals.profileForm.fields.subscriptionUrl') }}</label>
        <textarea
          v-model="formUrl"
          style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); box-sizing: border-box; min-height: 60px; font-family: inherit;"
        />
      </div>

      <div style="margin-top: 8px; margin-bottom: 8px;">
        <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('profiles.modals.profileForm.fields.userAgent') }}</label>
        <input
          v-model="formUserAgent"
          :placeholder="`clash-verge/v${version}`"
          style="width: 100%; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color); box-sizing: border-box;"
        />
      </div>

      <div style="margin-top: 8px; margin-bottom: 8px;">
        <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('profiles.modals.profileForm.fields.httpTimeout') }}</label>
        <div style="display: flex; align-items: center;">
          <input
            type="number"
            v-model="formTimeoutSeconds"
            placeholder="60"
            style="flex: 1; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
          />
          <span style="margin-left: 4px; font-size: 12px; white-space: nowrap;">{{ t('shared.units.seconds') }}</span>
        </div>
      </div>

      <div style="margin-top: 8px; margin-bottom: 8px;">
        <label style="display: block; margin-bottom: 4px; font-size: 14px;">{{ t('profiles.modals.profileForm.fields.updateInterval') }}</label>
        <div style="display: flex; align-items: center;">
          <input
            type="number"
            v-model="formUpdateInterval"
            style="flex: 1; padding: 8px; border: 1px solid var(--border-color); border-radius: 4px; background: var(--bg-color); color: var(--text-color);"
          />
          <span style="margin-left: 4px; font-size: 12px; white-space: nowrap;">{{ t('shared.units.minutes') }}</span>
        </div>
      </div>

      <div style="margin: 8px 0 8px 8px; display: flex; align-items: center; justify-content: space-between;">
        <label style="font-size: 14px;">{{ t('profiles.modals.profileForm.fields.useSystemProxy') }}</label>
        <input type="checkbox" v-model="formWithProxy" />
      </div>

      <div style="margin: 8px 0 8px 8px; display: flex; align-items: center; justify-content: space-between;">
        <label style="font-size: 14px;">{{ t('profiles.modals.profileForm.fields.useClashProxy') }}</label>
        <input type="checkbox" v-model="formSelfProxy" />
      </div>

      <div style="margin: 8px 0 8px 8px; display: flex; align-items: center; justify-content: space-between;">
        <label style="font-size: 14px;">{{ t('profiles.modals.profileForm.fields.acceptInvalidCerts') }}</label>
        <input type="checkbox" v-model="formAcceptInvalidCerts" />
      </div>

      <div style="margin: 8px 0 8px 8px; display: flex; align-items: center; justify-content: space-between;">
        <label style="font-size: 14px;">{{ t('profiles.modals.profileForm.fields.allowAutoUpdate') }}</label>
        <input type="checkbox" v-model="formAllowAutoUpdate" />
      </div>
    </template>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import { BaseDialog } from '@/components/base'
import { useProfiles } from '@/hooks/use-profiles'
import { createProfile, patchProfile } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { version } from '@root/package.json'
import FileInput from './file-input.vue'

const props = defineProps<{
  onChange: (isActivating?: boolean) => void
}>()

const { t } = useI18n()
const open = ref(false)
const openType = ref<'new' | 'edit'>('new')
const loading = ref(false)
const { profiles } = useProfiles()

const fileDataRef = ref<string | null>(null)

const formType = ref('remote')
const formName = ref('')
const formDesc = ref('')
const formUrl = ref('')
const formUserAgent = ref('')
const formTimeoutSeconds = ref<number>()
const formUpdateInterval = ref<number>()
const formWithProxy = ref(false)
const formSelfProxy = ref(false)
const formAcceptInvalidCerts = ref(false)
const formAllowAutoUpdate = ref(false)

const isRemote = computed(() => formType.value === 'remote')
const isLocal = computed(() => formType.value === 'local')

watch(formSelfProxy, (val) => { if (val) formWithProxy.value = false })
watch(formWithProxy, (val) => { if (val) formSelfProxy.value = false })

const create = () => {
  openType.value = 'new'
  formType.value = 'remote'
  formName.value = ''
  formDesc.value = ''
  formUrl.value = ''
  formUserAgent.value = ''
  formTimeoutSeconds.value = undefined
  formUpdateInterval.value = undefined
  formWithProxy.value = false
  formSelfProxy.value = false
  formAcceptInvalidCerts.value = false
  formAllowAutoUpdate.value = false
  open.value = true
}

const edit = (item: IProfileItem) => {
  formType.value = item.type || 'remote'
  formName.value = item.name || ''
  formDesc.value = item.desc || ''
  formUrl.value = item.url || ''
  formUserAgent.value = item.option?.user_agent || ''
  formTimeoutSeconds.value = item.option?.timeout_seconds
  formUpdateInterval.value = item.option?.update_interval
  formWithProxy.value = !!item.option?.with_proxy
  formSelfProxy.value = !!item.option?.self_proxy
  formAcceptInvalidCerts.value = !!item.option?.danger_accept_invalid_certs
  formAllowAutoUpdate.value = !!item.option?.allow_auto_update
  openType.value = 'edit'
  open.value = true
}

const onFileInput = (file: File, val: string) => {
  if (!formName.value) formName.value = file.name
  fileDataRef.value = val
}

let okLock = false
const handleOk = async () => {
  if (okLock || loading.value) return
  okLock = true
  loading.value = true
  try {
    if (!formType.value) throw new Error(t('profiles.modals.profileForm.errors.typeRequired'))
    if (formType.value === 'remote' && !formUrl.value) throw new Error(t('profiles.modals.profileForm.errors.urlRequired'))

    const option: any = {}
    if (formTimeoutSeconds.value) option.timeout_seconds = +formTimeoutSeconds.value
    if (formUpdateInterval.value) option.update_interval = +formUpdateInterval.value
    if (formUserAgent.value) option.user_agent = formUserAgent.value
    option.with_proxy = formWithProxy.value
    option.self_proxy = formSelfProxy.value
    option.danger_accept_invalid_certs = formAcceptInvalidCerts.value
    option.allow_auto_update = formAllowAutoUpdate.value

    const name = formName.value || `${formType.value} file`
    const item: IProfileItem = {
      type: formType.value as 'remote' | 'local',
      name,
      desc: formDesc.value,
      url: formUrl.value,
      option,
    }

    const isUpdate = openType.value === 'edit'
    const isActivating = isUpdate && profiles.value?.current === formName.value

    if (!isRemote.value) {
      if (openType.value === 'new') await createProfile(item, fileDataRef.value)
      else await patchProfile(formName.value, item)
    } else {
      try {
        if (openType.value === 'new') await createProfile(item, fileDataRef.value)
        else await patchProfile(formName.value, item)
      } catch {
        showNotice.info('profiles.modals.profileForm.feedback.notifications.creationRetry')
        const retryItem = { ...item, option: { ...item.option, with_proxy: false, self_proxy: true } }
        if (openType.value === 'new') await createProfile(retryItem, fileDataRef.value)
        else {
          await patchProfile(formName.value, retryItem)
          await patchProfile(formName.value, { option: { with_proxy: formWithProxy.value, self_proxy: formSelfProxy.value } })
        }
        showNotice.success('profiles.modals.profileForm.feedback.notifications.creationSuccess')
      }
    }

    open.value = false
    setTimeout(() => {
      fileDataRef.value = null
      props.onChange(isActivating)
    }, 500)
  } catch (err) {
    showNotice.error('profiles.modals.profileForm.errors.saveFailed', err)
  } finally {
    loading.value = false
    okLock = false
  }
}

const handleClose = () => {
  open.value = false
  fileDataRef.value = null
}

defineExpose({ create, edit })
</script>
