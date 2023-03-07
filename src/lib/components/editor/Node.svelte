<script>
  import { tick } from "svelte";
  import Fragment from "./Fragment.svelte";

  export let lines;
  let fragments = [];

  $: lines, lineToFragment()

  let lineToFragment = () => {
    fragments = []
    lines.forEach((line, i) => {
      fragments.push({
        key: i,
        content: line
      })
    fragments = fragments
    })
  }
  
  let handleNewFragment = async (key) => {
    lines.splice(key+1, 0, "")
    lines = lines
    await tick()
    document.getElementById(`frag-${key+1}`).lastChild.focus()
  }

  let handleDeleteFragment = (key) => {
    lines.splice(key, 1)
    lines = lines
    let el = document.getElementById(`frag-${key-1}`)
    el.lastChild.focus()
  }

  let handleMergeFragments = (key) => {
    let prev = document.getElementById(`frag-${key-1}`).lastChild
    let cur = document.getElementById(`frag-${key}`).lastChild
    prev.value += ' ' + cur.value
    lines.splice(key, 1)
    lines = lines
    prev.focus()
  }

  let handleFocusFragment = (key) => {
    document.getElementById(`frag-${key}`).lastChild.focus()
  }
</script>

<section id="node">
  {#each fragments as fragment}
    <div key={fragment.key}>
      <Fragment 
        key={fragment.key} 
        content={fragment.content} 
        handleNewFragment={handleNewFragment}
        handleDeleteFragment={handleDeleteFragment}
        handleMergeFragments={handleMergeFragments}
        handleFocusFragment={handleFocusFragment}
      />
    </div>
  {/each}
</section>
