import { writable } from "svelte/store"
import toml from "toml"
import json2toml from 'json2toml'
import { invoke } from '@tauri-apps/api/tauri'
import { configDir, resourceDir, appConfigDir } from '@tauri-apps/api/path'
import { exists, BaseDirectory, createDir, readTextFile, writeTextFile } from '@tauri-apps/api/fs'

// get existing user config or create empty one
let getConfig = async () => {
  let confExists = await exists("config/config.toml", { dir: BaseDirectory.AppConfig })
  if (!confExists) {
    await createDir('config', { dir: BaseDirectory.AppConfig, recursive: true })
    let template = await readTextFile('configTemplate.toml', { dir: BaseDirectory.Resource })
    await writeTextFile('config/config.toml', template, { dir: BaseDirectory.AppConfig })
  }
  let confStr = await readTextFile('config/config.toml', { dir: BaseDirectory.AppConfig })
  return toml.parse(confStr)
}

let updateConfig = async (userConfig) => {
  console.log(userConfig)
  await writeTextFile('config/config.toml', json2toml(userConfig), { dir: BaseDirectory.AppConfig })
}

const user = writable({
  config: await getConfig(),
});

export { user, updateConfig }
