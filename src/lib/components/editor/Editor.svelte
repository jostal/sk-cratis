<script>
  import { editor, openNode } from "../../stores/EditorStore";
  let content;
  let lines = []

  $: $editor, getContent()

  let getContent = async () => {
    content = await openNode($editor.nodePath, $editor.activeNode).then(res => res.content)
    if (content)
      lines = content.split(/\r?\n/)
  }
</script>

<section id="editor">
  <h1>
    <input value={$editor.activeNode} />
  </h1>
  {#each lines as line}
    <input value={line} />
  {/each}
</section>

<style lang="scss">
  #editor { 
    padding: 1em;
    background-color: var(--sidebar-color);
    margin: 20px 10% 20px 10%;
  }
</style>
