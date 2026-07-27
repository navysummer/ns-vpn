import { ref, computed, watch } from 'vue'

const areOrdersEqual = (a: string[], b: string[]) =>
  a.length === b.length && a.every((value, index) => value === b[index])

const createNavLookup = <T extends { path: string }>(items: readonly T[]) => {
  const map = new Map(items.map((item) => [item.path, item] as const))
  const defaultOrder = items.map((item) => item.path)
  return { map, defaultOrder }
}

const resolveMenuOrder = <T extends { path: string }>(
  order: string[] | null | undefined,
  defaultOrder: string[],
  map: Map<string, T>,
) => {
  const seen = new Set<string>()
  const resolved: string[] = []

  if (Array.isArray(order)) {
    for (const path of order) {
      if (map.has(path) && !seen.has(path)) {
        resolved.push(path)
        seen.add(path)
      }
    }
  }

  for (const path of defaultOrder) {
    if (!seen.has(path)) {
      resolved.push(path)
      seen.add(path)
    }
  }

  return resolved
}

interface UseNavMenuOrderOptions<T extends { path: string }> {
  enabled: boolean | { value: boolean }
  items: readonly T[]
  storedOrder: string[] | null | undefined | { value: string[] | null | undefined }
  onOptimisticUpdate?: (order: string[]) => void
  onPersist: (order: string[]) => Promise<void>
}

export const useNavMenuOrder = <T extends { path: string }>({
  enabled,
  items,
  storedOrder,
  onOptimisticUpdate,
  onPersist,
}: UseNavMenuOrderOptions<T>) => {
  const enabledVal = computed(() => typeof enabled === 'boolean' ? enabled : enabled.value)
  const storedOrderVal = computed(() => storedOrder && typeof storedOrder === 'object' && 'value' in storedOrder ? (storedOrder as any).value : storedOrder)

  const { map: navItemMap, defaultOrder } = createNavLookup(items)

  const configMenuOrder = computed(() =>
    resolveMenuOrder(storedOrderVal.value, defaultOrder, navItemMap),
  )

  const menuOrder = ref<string[]>(configMenuOrder.value)

  watch(configMenuOrder, (newOrder) => {
    menuOrder.value = [...newOrder]
  }, { immediate: true })

  const isDefaultOrder = computed(() =>
    areOrdersEqual(menuOrder.value, defaultOrder),
  )

  const handleMenuDragEnd = async (event: { active: { id: string }; over: { id: string } | null }) => {
    if (!enabledVal.value) return

    const { active, over } = event
    if (!over || active.id === over.id) return

    const activeId = String(active.id)
    const overId = String(over.id)

    const oldIndex = menuOrder.value.indexOf(activeId)
    const newIndex = menuOrder.value.indexOf(overId)

    if (oldIndex === -1 || newIndex === -1) return

    const previousOrder = [...menuOrder.value]
    const nextOrder = [...menuOrder.value]
    nextOrder.splice(oldIndex, 1)
    nextOrder.splice(newIndex, 0, previousOrder[oldIndex])

    menuOrder.value = nextOrder
    onOptimisticUpdate?.(nextOrder)

    try {
      await onPersist(nextOrder)
    } catch (error) {
      console.error('Failed to update menu order:', error)
      menuOrder.value = previousOrder
      onOptimisticUpdate?.(previousOrder)
    }
  }

  const resetMenuOrder = async () => {
    if (isDefaultOrder.value) return

    const previousOrder = [...menuOrder.value]
    const nextOrder = [...defaultOrder]

    menuOrder.value = nextOrder
    onOptimisticUpdate?.(nextOrder)

    try {
      await onPersist(nextOrder)
    } catch (error) {
      console.error('Failed to reset menu order:', error)
      menuOrder.value = previousOrder
      onOptimisticUpdate?.(previousOrder)
    }
  }

  return {
    menuOrder,
    navItemMap,
    handleMenuDragEnd,
    isDefaultOrder,
    resetMenuOrder,
  }
}
