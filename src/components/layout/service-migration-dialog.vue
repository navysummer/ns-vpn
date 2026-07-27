<template>
  <BaseDialog
    :open="open"
    :title="t('layout.components.serviceMigration.title')"
    :ok-btn="okBtnText"
    :cancel-btn="t('layout.components.serviceMigration.continueSidecar')"
    :disable-ok="loading"
    :disable-cancel="loading"
    :loading="loading"
    @ok="handleServiceAction"
    @cancel="handleContinue"
  >
    <n-alert type="warning">
      {{ alertMessage }}
    </n-alert>
  </BaseDialog>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useTranslation } from '@/composables/use-i18n'
const { t } = useTranslation()
import BaseDialog from '@/components/base/base-dialog.vue'
import { runStateQueryKey } from '@/hooks/use-system-state'
import { useVisibility } from '@/hooks/use-visibility'
import {
  continueWithSidecar,
  getRuntimeState,
  installService,
  reinstallService,
  repairService,
  restartCore,
  type RunState,
} from '@/services/cmds'
import { showNotice } from '@/services/notice-service'
import { setCacheDataAsync, useQuery } from '@/services/query-client'

const pageVisible = useVisibility()
const loading = ref(false)
const stateRefreshFailed = ref(false)
const workflowIncomplete = ref(false)

const { data: runState } = useQuery({
  queryKey: runStateQueryKey,
  queryFn: getRuntimeState,
  enabled: true,
  retry: 1,
  refetchInterval: pageVisible ? 30000 : false,
})

const needsDecision = computed(() =>
  stateRefreshFailed.value || Boolean(runState.value?.serviceNeedsAttention),
)

const remedy = computed(() => {
  if (runState.value?.pendingAction === 'install') return 'install'
  if (stateRefreshFailed.value || runState.value?.service === 'unavailable') return 'repair'
  return 'reinstall'
})

const open = computed(() => loading.value || workflowIncomplete.value || needsDecision.value)
const showCheckingMessage = computed(() => loading.value || !needsDecision.value)

const okBtnText = computed(() => {
  const r = remedy.value
  return t(
    r === 'install'
      ? 'settings.sections.proxyControl.actions.installService'
      : r === 'repair'
        ? 'layout.components.serviceMigration.repair'
        : 'layout.components.serviceMigration.reinstall',
  )
})

const alertMessage = computed(() => {
  if (showCheckingMessage.value) {
    return t('layout.components.serviceMigration.checkingMessage')
  }
  return t(
    remedy.value === 'reinstall'
      ? 'layout.components.serviceMigration.message'
      : 'layout.components.serviceMigration.unavailableMessage',
  )
})

const refreshRunState = async () => {
  try {
    const data = await getRuntimeState()
    await setCacheDataAsync<RunState>(runStateQueryKey, data)
    stateRefreshFailed.value = false
    return data
  } catch (error) {
    stateRefreshFailed.value = true
    throw error
  }
}

const handleServiceAction = async () => {
  loading.value = true
  workflowIncomplete.value = true
  let actionSucceeded = false
  try {
    if (remedy.value === 'install') {
      await installService()
    } else if (remedy.value === 'repair') {
      await repairService()
    } else {
      await reinstallService()
    }
    actionSucceeded = true
  } catch (error) {
    showNotice.error('layout.components.serviceMigration.errors.actionFailed', error)
  }

  let initialRefreshSucceeded = false
  try {
    await refreshRunState()
    initialRefreshSucceeded = true
  } catch (error) {
    showNotice.error('layout.components.serviceMigration.errors.stateRefreshFailed', error)
  }
  if (!actionSucceeded || !initialRefreshSucceeded) {
    loading.value = false
    return
  }

  let restartSucceeded = false
  try {
    await restartCore()
    restartSucceeded = true
  } catch (error) {
    showNotice.error('layout.components.serviceMigration.errors.restartFailed', error)
  }

  let finalRefreshSucceeded = false
  try {
    await refreshRunState()
    finalRefreshSucceeded = true
  } catch (error) {
    showNotice.error('layout.components.serviceMigration.errors.stateRefreshFailed', error)
  }
  if (restartSucceeded && finalRefreshSucceeded) {
    workflowIncomplete.value = false
    showNotice.success('layout.components.serviceMigration.success')
  }
  loading.value = false
}

const handleContinue = async () => {
  loading.value = true
  workflowIncomplete.value = true
  let startupError: unknown
  try {
    await continueWithSidecar()
  } catch (error) {
    startupError = error
  }
  let installRefreshSucceeded = false
  try {
    await refreshRunState()
    installRefreshSucceeded = true
  } catch (error) {
    showNotice.error('layout.components.serviceMigration.errors.stateRefreshFailed', error)
  }
  if (startupError) {
    showNotice.error('layout.components.serviceMigration.errors.sidecarFailed', startupError)
  } else if (installRefreshSucceeded) {
    workflowIncomplete.value = false
  }
  loading.value = false
}
</script>
