<script>
  import { tick } from "svelte";
  import Fragment from "./Fragment.svelte";
  import { editor } from "../../stores/EditorStore";
  import { user } from "../../stores/UserStore"
  import { parseContent, saveNode } from "../../utils/utils.editor.js"
  export let lines
  let fragments = []
  let dragging = false
  export let nodeName
  let nodePath = $editor.isJournal ? $user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/' + nodeName + '.md' : $editor.nodePath

  $: $editor, nodePath = $editor.isJournal ? $user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/' + nodeName + '.md' : $editor.nodePath
  $: lines, lineToFragment()

  let lineToFragment = async () => {
    fragments = new Array(lines.length)

    for (let i = 0;i < lines.length; i++) {
      let parsedContent = await parseContent(lines[i])
      fragments[i] = {
        key: i,
        level: parsedContent.level,
        content: parsedContent.content,
        active: false
      }
      fragments = fragments
    }
  }

</script>

<section id="node">
  <h1>{nodeName}</h1>
  {#if fragments[fragments.length - 1]}
    {#each fragments as fragment}
      <div key={fragment.key}>
        <Fragment 
          key={fragment.key} 
          level={fragment.level}
          content={fragment.content}
          active={fragment.active}
          bind:fragments={fragments}
          bind:dragging={dragging}
          saveNode={(fragments) => {saveNode(fragments, nodePath)}}
        />
      </div>
    {/each}
  {/if}
</section>

<style lang="scss">
  #node {
    div {
      pointer-events: none;
    }
  }
</style>
