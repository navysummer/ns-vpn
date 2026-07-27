<template>
  <BaseDialog
    :open="open"
    :title="openType === 'new' ? t('tests.modals.test.title.create') : t('tests.modals.test.title.edit')"
    :content-sx="{ width: '375px', paddingBottom: 0, maxHeight: '80%' }"
    :ok-btn="t('shared.actions.save')"
    :cancel-btn="t('shared.actions.cancel')"
    :loading="loading"
    :on-close="handleClose"
    :on-cancel="handleClose"
    :on-ok="handleOk"
  >
    <n-input
      v-model:value="form.name"
      :placeholder="t('shared.labels.name')"
      size="small"
      style="margin-top: 16px; margin-bottom: 16px;"
    />
    <n-input
      v-model:value="form.icon"
      :placeholder="t('shared.labels.icon')"
      type="textarea"
      :autosize="{ minRows: 2, maxRows: 5 }"
      size="small"
      style="margin-bottom: 16px;"
    />
    <n-input
      v-model:value="form.url"
      :placeholder="t('tests.modals.test.fields.url')"
      type="textarea"
      :autosize="{ minRows: 2, maxRows: 3 }"
      size="small"
      style="margin-bottom: 16px;"
    />
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
import { nanoid } from 'nanoid'

import { BaseDialog } from '@/components/base'
import { useVerge } from '@/hooks/use-verge'
import { showNotice } from '@/services/notice-service'

interface Props {
  onChange: (uid: string, patch?: Partial<IVergeTestItem>) => void
}

const props = defineProps<Props>()

const { t } = useTranslation()
const open = ref(false)
const openType = ref<'new' | 'edit'>('new')
const loading = ref(false)
const okLock = ref(false)
const { verge, patchVerge } = useVerge()
const testList = verge?.test_list ?? []

const form = reactive<IVergeTestItem>({
  uid: '',
  name: '',
  icon: '',
  url: '',
})

const patchTestList = async (uid: string, patch: Partial<IVergeTestItem>) => {
  const newList = testList.map((x: IVergeTestItem) => {
    if (x.uid === uid) {
      return { ...x, ...patch }
    }
    return x
  })
  await patchVerge({ test_list: newList })
}

const setForm = (item: IVergeTestItem) => {
  form.uid = item.uid || ''
  form.name = item.name || ''
  form.icon = item.icon || ''
  form.url = item.url || ''
}

const resetForm = () => {
  form.uid = ''
  form.name = ''
  form.icon = ''
  form.url = ''
}

const create = () => {
  openType.value = 'new'
  resetForm()
  open.value = true
}

const edit = (item: IVergeTestItem) => {
  if (item) {
    setForm(item)
  }
  openType.value = 'edit'
  open.value = true
}

defineExpose({ create, edit })

const handleOk = async () => {
  if (okLock.value) return
  okLock.value = true
  loading.value = true
  try {
    if (!form.name) {
      throw new Error(t('tests.modals.test.errors.nameRequired'))
    }
    if (!form.url) {
      throw new Error(t('tests.modals.test.errors.urlRequired'))
    }

    if (form.icon && form.icon.startsWith('<svg')) {
      form.icon = form.icon.replace(/<!--[\s\S]*?-->/g, '')
      const doc = new DOMParser().parseFromString(
        form.icon,
        'image/svg+xml',
      )
      if (doc.querySelector('parsererror')) {
        throw new Error(t('tests.modals.test.errors.invalidSvg'))
      }
    }

    if (openType.value === 'new') {
      const uid = nanoid()
      const item = { ...form, uid }
      const newList = [...testList, item]
      await patchVerge({ test_list: newList })
      props.onChange(uid)
    } else {
      if (!form.uid) {
        throw new Error(t('tests.modals.test.errors.uidMissing'))
      }
      const uid = form.uid
      await patchTestList(uid, form)
      props.onChange(uid, form)
    }
    open.value = false
    loading.value = false
    setTimeout(() => resetForm(), 500)
  } catch (err: any) {
    showNotice.error('tests.modals.test.errors.saveFailed', err)
    loading.value = false
  } finally {
    okLock.value = false
  }
}

const handleClose = () => {
  open.value = false
  setTimeout(() => resetForm(), 500)
}
</script>
