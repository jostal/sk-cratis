import { invoke } from '@tauri-apps/api/tauri'

async function createNetwork(location, name) {
  await invoke('create_network', { location, name })
}

async function createNode(location, name) {
  await invoke('create_node', { location, name })
}

export { createNetwork, createNode }
