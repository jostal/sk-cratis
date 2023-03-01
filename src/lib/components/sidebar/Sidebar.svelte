<script>
  import { user } from "../../stores/UserStore.js";
  import { editor, openNode } from "../../stores/EditorStore.js";
  import { createNode } from "../../utils/utils.network";
  let showNodeDialog = false;

  let handleCreateNode = (data) => {
    let formFields = data.elements;
    createNode($user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/', formFields.nodeName.value);
    showNodeDialog = false;
    $editor.activeNode = formFields.nodeName.value + '.md'
  }

  $: $editor, openNode($user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/' + $editor.activeNode, $editor.activeNode)
</script>

<section id="sidebar">
  <button on:click={() => showNodeDialog = true}>Create Node</button>
  {#if showNodeDialog}
    <div id="node-dialog">
      <form on:submit|preventDefault={(e) => handleCreateNode(e.target)}>
        <label for="nodeName">Node name</label>
        <input name="nodeName"/>
        <button type="submit">Create Node</button>
      </form>
    </div>
  {/if}
</section>

<style lang="scss">
  #sidebar {
    position: sticky;
    left: 0;
    background-color: var(--sidebar-color);
    height: 100vh;
    max-width: 250px;
    width: 20%;
  }
</style>
