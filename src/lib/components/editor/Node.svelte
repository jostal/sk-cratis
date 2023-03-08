<script>
  import { tick } from "svelte";
  import Fragment from "./Fragment.svelte";
  import { editor } from "../../stores/EditorStore";
  import { parseContent } from "../../utils/utils.editor";
  export let lines;
  let fragments = [];

  $: lines, lineToFragment()

  let lineToFragment = async () => {
    fragments = []

    await lines.forEach(async (line, i) => {
      let parsedContent = await parseContent(line)
      fragments.push({
        key: i,
        level: parsedContent.level,
        content: parsedContent.content,
        html: parsedContent.html
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
    $editor.activeFragment = key
  }
</script>

<section id="node">
  <h1>{$editor.activeNode}</h1>
  {#each fragments as fragment}
    <div key={fragment.key}>
      <Fragment 
        key={fragment.key} 
        level={fragment.level}
        content={fragment.content} 
        html={fragment.html}
        handleNewFragment={handleNewFragment}
        handleDeleteFragment={handleDeleteFragment}
        handleMergeFragments={handleMergeFragments}
        handleFocusFragment={handleFocusFragment}
      />
    </div>
  {/each}
</section>
