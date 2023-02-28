import { invoke } from '@tauri-apps/api/tauri'

async function createNetwork(location, name) {
  await invoke('create_network', { location, name })
}

export { createNetwork }
