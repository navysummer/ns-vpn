<template>
  <div>
    <ProfileItem
      :selected="selected"
      :activating="activating"
      :item-data="itemData"
      :mutate-profiles="mutateProfiles"
      :on-select="onSelect"
      :on-edit="onEdit"
      :on-save="onSave"
      :on-delete="onDelete"
      :batch-mode="batchMode"
      :is-selected="isSelected"
      :on-selection-change="onSelectionChange"
      :timer-update-revision="timerUpdateRevision"
      :completed-update-revision="completedUpdateRevision"
    />
  </div>
</template>

<script setup lang="ts">
import type { ProfileItemProps } from './profile-item.vue'
import ProfileItem from './profile-item.vue'

interface SortableProfileItemProps extends /* @vue-ignore */ Omit<ProfileItemProps, 'dragHandleRef' | 'dragHandleAttributes' | 'dragHandleListeners'> {
  id: string
}

defineProps<SortableProfileItemProps>()
</script>
