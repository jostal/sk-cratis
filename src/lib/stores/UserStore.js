import { writable } from "svelte/store"
import toml from "toml"
import json2toml from 'json2toml'
import { invoke } from '@tauri-apps/api/tauri'
import { configDir, resourceDir, appConfigDir, resolveResource } from '@tauri-apps/api/path'
import { exists, BaseDirectory, createDir, readTextFile, writeTextFile } from '@tauri-apps/api/fs'

// get existing user config or create empty one
let getConfig = async () => {
  let confExists = await exists("config/config.toml", { dir: BaseDirectory.AppConfig })
  if (!confExists) {
    await createDir('config', { dir: BaseDirectory.AppConfig, recursive: true })
    let resPath = await resolveResource('templates/defaultUserConfig.toml')
    let template = await readTextFile(resPath)
    await writeTextFile('config/config.toml', template, { dir: BaseDirectory.AppConfig })
  }
  let confStr = await readTextFile('config/config.toml', { dir: BaseDirectory.AppConfig })
  return toml.parse(confStr)
}

let updateConfig = async (userConfig) => {
  if (userConfig)
    await writeTextFile('config/config.toml', json2toml(userConfig), { dir: BaseDirectory.AppConfig })
}



const user = writable({
  config: null,
});

export { user, updateConfig, getConfig }
