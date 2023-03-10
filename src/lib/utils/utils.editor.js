import { invoke } from '@tauri-apps/api'

async function convertMarkdown(content) {
  return await invoke('parse_md', { content })
}

async function parseContent(line) {
  let splitArr = line.split('-')
  let level = 0
  if (splitArr.length > 1) {
    level = splitArr[0].length/4
  }
  let html = await convertMarkdown(line.substring(line.indexOf("-") + 2, line.length))
  return {
    level: level,
    content: line.substring(line.indexOf("-")+2, line.length),
    html: html
  }
}

async function saveNode(fragments, nodePath) {
  //write fragments to file with bullet at appropriate level
  let nodeStr = ""
  fragments.forEach(frag => {
    if (frag.content !== "")
      nodeStr += "    ".repeat(frag.level) + "- " + frag.content + "\n"
  })
  
  await invoke('save_node', { nodeStr, nodePath })
}

export { parseContent, convertMarkdown, saveNode }
