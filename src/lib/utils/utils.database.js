import { invoke } from "@tauri-apps/api";
import { BaseDirectory, exists, writeTextFile } from "@tauri-apps/api/fs";
import { configDir } from "@tauri-apps/api/path";

async function createDatabase() {
  let dbExists = await exists("database.db", { dir: BaseDirectory.AppConfig })
  if (!dbExists) {
    await writeTextFile("database.db", "", { dir: BaseDirectory.AppConfig })
  }
  let databaseDir = await configDir()
  databaseDir += "/Cratis/database.db"
  await invoke('create_database', { databaseDir })
}

async function indexNodes(nodesDir) {
  let databaseDir = await configDir()
  databaseDir += "/Cratis/database.db"
  await invoke('index_nodes', { databaseDir, nodesDir })
}

async function addNode(nodeName) {
  let databaseDir = await configDir()
  databaseDir += "/Cratis/database.db"
  await invoke('add_node', { databaseDir, nodeName })
}

async function updateReferences(nodePath) {
  let databaseDir = await configDir()
  databaseDir += "/Cratis/database.db"
  await invoke('update_references', { databaseDir, nodePath })
}

export { createDatabase, indexNodes, addNode, updateReferences }
