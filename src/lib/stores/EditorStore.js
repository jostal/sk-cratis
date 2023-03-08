import { writable } from 'svelte/store'
import { invoke } from '@tauri-apps/api/tauri';

const editor = writable({
  activeNode: "",
  activeFragment: 0,
  nodePath: ""
});

let openNode = async (nodePath, nodeName) => {
  let nodeStr;
  if (nodeName !== "") {
    nodeStr = await invoke('open_node', { nodePath })  
  }
  return {
    content: nodeStr
  }
}

export { editor, openNode }
