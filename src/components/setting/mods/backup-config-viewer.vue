<script setup lang="ts">
import { computed, ref, watch, onMounted } from 'vue'
import i18n from 'i18next'

import { useVerge } from '@/hooks/use-verge'
import { saveWebdavConfig, createWebdavBackup } from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import {
  buildWebdavSignature,
  getWebdavStatus,
  setWebdavStatus,
} from '@/services/webdav-status'
import { isValidUrl } from '@/utils/network'

const props = defineProps<{
  onBackupSuccess: () => Promise<void>
  onSaveSuccess: (signature?: string) => Promise<void>
  onRefresh: () => Promise<void>
  onInit: () => Promise<void>
  setLoading: (loading: boolean) => void
}>()

const { verge, mutateVerge } = useVerge()
const { webdav_url, webdav_username, webdav_password } = verge || {}

const showPassword = ref(false)
const urlRef = ref<HTMLInputElement>()
const usernameRef = ref<HTMLInputElement>()
const passwordRef = ref<HTMLInputElement>()

const formValues = ref({
  url: webdav_url || '',
  username: webdav_username || '',
  password: webdav_password || '',
})

const webdavChanged = ref(false)

watch(
  () => [formValues.value.url, formValues.value.username, formValues.value.password],
  ([url, username, password]) => {
    webdavChanged.value =
      webdav_url !== url ||
      webdav_username !== username ||
      webdav_password !== password
  },
  { deep: true, immediate: true }
)

const webdavSignature = computed(() => buildWebdavSignature(verge))
const webdavStatus = computed(() => getWebdavStatus(webdavSignature.value))
const shouldAutoInit = computed(() => webdavStatus.value !== 'failed')

const handleClickShowPassword = () => {
  showPassword.value = !showPassword.value
}

onMounted(() => {
  if (webdav_url && webdav_username && webdav_password && shouldAutoInit.value) {
    props.onInit()
  }
})

const checkForm = () => {
  const username = usernameRef.value?.value
  const password = passwordRef.value?.value
  const url = urlRef.value?.value

  if (!url) {
    urlRef.value?.focus()
    showNotice.error('settings.modals.backup.messages.webdavUrlRequired')
    throw new Error(i18n.t('settings.modals.backup.messages.webdavUrlRequired'))
  } else if (!isValidUrl(url)) {
    urlRef.value?.focus()
    showNotice.error('settings.modals.backup.messages.invalidWebdavUrl')
    throw new Error(i18n.t('settings.modals.backup.messages.invalidWebdavUrl'))
  }
  if (!username) {
    usernameRef.value?.focus()
    showNotice.error('settings.modals.backup.messages.usernameRequired')
    throw new Error(i18n.t('settings.modals.backup.messages.usernameRequired'))
  }
  if (!password) {
    passwordRef.value?.focus()
    showNotice.error('settings.modals.backup.messages.passwordRequired')
    throw new Error(i18n.t('settings.modals.backup.messages.passwordRequired'))
  }
}

const save = async () => {
  checkForm()
  const data = formValues.value
  const signature = buildWebdavSignature({
    webdav_url: data.url,
    webdav_username: data.username,
    webdav_password: data.password,
  })
  const trimmedUrl = data.url.trim()
  const trimmedUsername = data.username.trim()

  try {
    props.setLoading(true)
    await saveWebdavConfig(trimmedUrl, trimmedUsername, data.password)
    await mutateVerge(
      (current: any) =>
        current
          ? {
              ...current,
              webdav_url: trimmedUrl,
              webdav_username: trimmedUsername,
              webdav_password: data.password,
            }
          : current,
      false,
    )
    setWebdavStatus(signature, 'unknown')
    showNotice.success('settings.modals.backup.messages.webdavConfigSaved')
    await props.onSaveSuccess(signature)
  } catch (error) {
    showNotice.error(
      'settings.modals.backup.messages.webdavConfigSaveFailed',
      { error },
      3000,
    )
  } finally {
    props.setLoading(false)
  }
}

const handleBackup = async () => {
  checkForm()
  const signature = buildWebdavSignature({
    webdav_url: formValues.value.url,
    webdav_username: formValues.value.username,
    webdav_password: formValues.value.password,
  })

  try {
    props.setLoading(true)
    await createWebdavBackup().then(async () => {
      showNotice.success('settings.modals.backup.messages.backupCreated')
      await props.onBackupSuccess()
    })
    setWebdavStatus(signature, 'ready')
  } catch (error) {
    showNotice.error('settings.modals.backup.messages.backupFailed', {
      error,
    })
    setWebdavStatus(signature, 'failed')
  } finally {
    props.setLoading(false)
  }
}

const isChanged = computed(() =>
  webdavChanged.value ||
  webdav_url === undefined ||
  webdav_username === undefined ||
  webdav_password === undefined
)
</script>

<template>
  <form @submit.prevent>
    <div class="MuiGrid-root MuiGrid-container" style="display: flex; flex-wrap: wrap; gap: 16px;">
      <div style="flex: 0 0 75%; max-width: 75%;">
        <div style="display: flex; flex-wrap: wrap; gap: 16px;">
          <div style="flex: 0 0 100%;">
            <div class="MuiFormControl-root MuiTextField-root" style="width: 100%;">
              <input
                ref="urlRef"
                :value="formValues.url"
                @input="formValues.url = ($event.target as HTMLInputElement).value"
                :placeholder="i18n.t('settings.modals.backup.fields.webdavUrl')"
                class="MuiInput-root MuiInput-sizeSmall"
                style="width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; margin-top: 8px;"
                autocomplete="off"
                spellcheck="false"
              />
            </div>
          </div>
          <div style="flex: 0 0 50%;">
            <input
              ref="usernameRef"
              :value="formValues.username"
              @input="formValues.username = ($event.target as HTMLInputElement).value"
              :placeholder="i18n.t('settings.modals.backup.fields.username')"
              class="MuiInput-root MuiInput-sizeSmall"
              style="width: 100%; padding: 8px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box;"
              autocomplete="off"
              spellcheck="false"
            />
          </div>
          <div style="flex: 0 0 50%;">
            <div style="display: flex; align-items: center; border: 1px solid #ccc; border-radius: 4px; padding: 0 8px;">
              <input
                ref="passwordRef"
                :value="formValues.password"
                @input="formValues.password = ($event.target as HTMLInputElement).value"
                :type="showPassword ? 'text' : 'password'"
                :placeholder="i18n.t('shared.labels.password')"
                style="flex: 1; padding: 8px 0; border: none; outline: none;"
                autocomplete="off"
                spellcheck="false"
              />
              <button class="MuiIconButton-root MuiIconButton-sizeSmall" type="button" @click="handleClickShowPassword">
                <svg v-if="showPassword" viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M12 7c2.76 0 5 2.24 5 5 0 .65-.13 1.26-.36 1.83l2.92 2.92c1.51-1.26 2.7-2.89 3.43-4.75-1.73-4.39-6-7.5-11-7.5-1.4 0-2.74.25-3.98.7l2.16 2.16C10.74 7.13 11.35 7 12 7zM2 4.27l2.28 2.28.46.46C3.08 8.3 1.78 10.02 1 12c1.73 4.39 6 7.5 11 7.5 1.55 0 3.03-.3 4.38-.84l.42.42L19.73 22 21 20.73 3.27 3 2 4.27zM7.53 9.8l1.55 1.55c-.05.21-.08.43-.08.65 0 1.66 1.34 3 3 3 .22 0 .44-.03.65-.08l1.55 1.55c-.67.33-1.41.53-2.2.53-2.76 0-5-2.24-5-5 0-.79.2-1.53.53-2.2zm4.31-.78l3.15 3.15.02-.16c0-1.66-1.34-3-3-3l-.17.01z"/></svg>
                <svg v-else viewBox="0 0 24 24" width="1em" height="1em" fill="currentColor"><path d="M12 4.5C7 4.5 2.73 7.61 1 12c1.73 4.39 6 7.5 11 7.5s9.27-3.11 11-7.5c-1.73-4.39-6-7.5-11-7.5zM12 17c-2.76 0-5-2.24-5-5s2.24-5 5-5 5 2.24 5 5-2.24 5-5 5zm0-8c-1.66 0-3 1.34-3 3s1.34 3 3 3 3-1.34 3-3-1.34-3-3-3z"/></svg>
              </button>
            </div>
          </div>
        </div>
      </div>
      <div style="flex: 0 0 25%; max-width: 25%;">
        <div style="display: flex; flex-direction: column; justify-content: space-between; align-items: stretch; height: 100%;">
          <template v-if="isChanged">
            <button
              class="MuiButton-root MuiButton-contained MuiButton-containedPrimary"
              type="button"
              style="height: 100%; padding: 12px;"
              @click="save"
            >
              {{ i18n.t('shared.actions.save') }}
            </button>
          </template>
          <template v-else>
            <button
              class="MuiButton-root MuiButton-contained MuiButton-containedSuccess"
              type="button"
              style="padding: 12px; margin-bottom: 8px;"
              @click="handleBackup"
            >
              {{ i18n.t('settings.modals.backup.actions.backup') }}
            </button>
            <button
              class="MuiButton-root MuiButton-outlined"
              type="button"
              style="padding: 12px;"
              @click="props.onRefresh"
            >
              {{ i18n.t('shared.actions.refresh') }}
            </button>
          </template>
        </div>
      </div>
    </div>
  </form>
</template>
