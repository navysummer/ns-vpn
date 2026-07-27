import { listen, EventCallback } from '@tauri-apps/api/event'


export const useListen = () => {
  const addListener = async <T>(eventName: string, handler: EventCallback<T>) => {
    return await listen(eventName, handler)
  }

  return {
    addListener,
  }
}
