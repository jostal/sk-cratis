import { invoke } from '@tauri-apps/api/tauri'
import { addNode } from './utils.database'

async function createNetwork(location, name) {
  await invoke('create_network', { location, name })
}

async function createNode(location, name) {
  await invoke('create_node', { location, name })
  addNode(name)
}

async function getJournalEntries(journalDir) {
  return await invoke('get_journal_entries', { journalDir })
}

export { createNetwork, createNode, getJournalEntries }
