<script>
  import { editor, openNode } from "../../stores/EditorStore";
  let content;
  console.log($editor)

  $: $editor, getContent()

  let getContent = async () => {
   content = await openNode($editor.nodePath, $editor.activeNode).then(res => res.content)
  }
</script>

<section id="editor">
  {content} 
</section>

<style lang="scss">
  #editor { 
    padding: 1em;
    background-color: var(--sidebar-color);
    margin: 20px 10% 20px 10%;
  }
</style>
