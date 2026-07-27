<script setup lang="ts">
import { ref, useSlots } from 'vue'
import noop from '@/utils/noop'

const props = withDefaults(defineProps<{
  value?: any
  valueProps?: string
  onChangeProps?: string
  waitTime?: number
  onChange?: (value: any) => void
  onFormat?: (...args: any[]) => any
  onGuard?: (value: any, oldValue: any) => Promise<void>
  onCatch?: (error: Error) => void
}>(), {
  valueProps: 'value',
  onChangeProps: 'onChange',
  waitTime: 0,
  onGuard: noop,
  onCatch: noop,
  onChange: noop,
})

const emit = defineEmits<{
  change: [value: any]
  guard: [value: any]
}>()

const slots = useSlots()
const lockRef = ref(false)
const saveRef = ref(props.value)
const lastRef = ref(0)
const timeRef = ref<any>(undefined)

const childHandler = async (...args: any[]) => {
  if (lockRef.value) return
  lockRef.value = true

  try {
    const newValue = props.onFormat ? props.onFormat(...args) : args[0]
    const onChange = props.onChange || ((v: any) => emit('change', v))
    onChange(newValue)

    const now = Date.now()
    if (props.waitTime <= 0 || now - lastRef.value >= props.waitTime) {
      saveRef.value = props.value
    }
    lastRef.value = now

    if (props.waitTime <= 0) {
      const onGuard = props.onGuard || ((v: any) => emit('guard', v))
      await onGuard(newValue, props.value)
      lockRef.value = false
    } else {
      clearTimeout(timeRef.value)
      timeRef.value = setTimeout(async () => {
        try {
          const onGuard = props.onGuard || ((v: any) => emit('guard', v))
          await onGuard(newValue, saveRef.value)
        } catch (err: any) {
          const onChange = props.onChange || ((v: any) => emit('change', v))
          onChange(saveRef.value)
          props.onCatch(err)
        } finally {
          lockRef.value = false
        }
      }, props.waitTime)
    }
  } catch (err: any) {
    const onChange = props.onChange || ((v: any) => emit('change', v))
    onChange(saveRef.value)
    props.onCatch(err)
    lockRef.value = false
  }
}
</script>

<template>
  <component :is="slots.default?.()[0]?.type" v-bind="{ ...slots.default?.()[0]?.props, [valueProps]: value, [onChangeProps]: childHandler }">
    <template v-for="(_, name) in slots.default?.()[0]?.children" #[name]="slotProps">
      <slot :name="name" v-bind="slotProps" />
    </template>
  </component>
</template>
