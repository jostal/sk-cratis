<script>
  import { editor, openNode } from "../../stores/EditorStore";
  import Node from "./Node.svelte";
  let content;
  let lines = []

  $: $editor.activeNode, getContent()

  let getContent = async () => {
    lines = []
    content = await openNode($editor.nodePath, $editor.activeNode).then(res => res.content)
    if (content) {
      lines = content.split(/\r?\n/)
    } else {
      lines = [""]
    }
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
    height: calc(100% - 5em);
 }
</style>
