import { writable } from 'svelte/store'

const editor = writable({
  activeNode: ""
});

export { editor }
