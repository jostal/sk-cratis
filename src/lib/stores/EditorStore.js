import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/tauri';

let openNode = async (nodePath, nodeName) => {
  let nodeStr;
  if (nodeName !== "") {
    console.log("AAA")
    nodeStr = await invoke('open_node', { nodePath })  
  }
  console.log(nodeStr)
}

const editor = writable({
  activeNode: ""
});

export { editor, openNode }
