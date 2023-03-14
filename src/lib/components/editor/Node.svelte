<script>
  import { tick } from "svelte";
  import Fragment from "./Fragment.svelte";
  import { editor } from "../../stores/EditorStore";
  import { parseContent } from "../../utils/utils.editor.js"
  export let lines
  let fragments = []
  let dragging = false

  $: lines, lineToFragment()

  let lineToFragment = async () => {
    fragments = []

    await lines.forEach( async (line, i) => {
      let parsedContent = await parseContent(line)
      fragments.push({
        key: i,
        level: parsedContent.level,
        content: parsedContent.content,
        active: false
      })
      fragments = fragments
    })
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
        active={fragment.active}
        bind:fragments={fragments}
        bind:dragging={dragging}
      />
    </div>
  {/each}
</section>

<style lang="scss">
  #node {
    div {
      pointer-events: none;
    }
  }
</style>
