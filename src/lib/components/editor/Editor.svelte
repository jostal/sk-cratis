<script>
  import { editor, openNode } from "../../stores/EditorStore";
  import Node from "./Node.svelte";
  let content;
  let lines = [""]

  $: $editor, getContent()

  let getContent = async () => {
    content = await openNode($editor.nodePath, $editor.activeNode).then(res => res.content)
    if (content)
      lines = content.split(/\r?\n/)
    lines = lines
  }

</script>

<section id="editor">
  <Node bind:lines={lines} /> 
</section>

<style lang="scss">
  #editor { 
    padding: 1em;
    background-color: var(--sidebar-color);
    margin: 20px 10% 20px 10%;
 }
</style>
