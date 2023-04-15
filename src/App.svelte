<script>
  import { user, updateConfig, getConfig } from './lib/stores/UserStore.js';
  import { editor } from './lib/stores/EditorStore.js'
  import { open } from '@tauri-apps/api/dialog';
  import { documentDir } from '@tauri-apps/api/path';
  import { createNetwork } from './lib/utils/utils.network.js';
  import { createDatabase } from './lib/utils/utils.database.js';
  import Sidebar from './lib/components/sidebar/Sidebar.svelte';
  import Editor from './lib/components/editor/Editor.svelte';
  import Navbar from './lib/components/nav/Navbar.svelte';
  import Graph from './lib/components/graph/Graph.svelte'

  let requestNetworkLocation
  let dir;

  let initUser = async () => {
    $user.config = await getConfig()      
    requestNetworkLocation = $user.config.network_config.location === "";
    return "ok"
  }

  let userInit = initUser()

  createDatabase()

  let handleNetworkSelection = async () => {
    dir = await open({
      directory: true,
      defaultPath: await documentDir(),
    });
  }

  let handleCreateNetwork = async (data) => {
    let formFields = data.elements;

    if (dir) {
      $user.config.network_config.location = dir 
      $user.config.network_config.name = formFields.networkName.value
      requestNetworkLocation = false
      createNetwork($user.config.network_config.location, $user.config.network_config.name)
    }
  }

  $: $user, updateConfig($user.config)

</script>

<main>
  {#await userInit then ok} 
    {#if requestNetworkLocation}
      <div id="reqNetworkContainer">
        <h1>Create or select an existing network</h1>
        <button on:click={() => handleNetworkSelection()}>Select Network Location</button>
        <p>Location: {dir ? dir : "Must select network location"}</p>
        <form on:submit|preventDefault={(e) => handleCreateNetwork(e.target)}>
          <label for="networkName">Network Name</label>
          <input name="networkName" required />
          <button type="submit">Create Network</button>
        </form>       
      </div>
    {:else}
      <div id="ui-shell">
        <div id="nav">
          <Navbar />
        </div>
        {#if $editor.showEditor}
          <div id="editor-container">
            <Editor />
          </div>
        {:else}
          <div id="graph-container">
            <Graph />
          </div>
        {/if}
      </div>
    {/if}
  {/await}
</main>

<style lang="scss">
  #ui-shell {
    position: relative;
    display: grid;
    grid-template-areas: 
      "nav nav"
      "editor editor";
    grid-template-columns: minmax(100px, 230px) calc(100% - 230px);
    grid-template-rows: var(--nav-height) calc(100% - var(--nav-height));

    #nav {
      grid-area: nav;
      position: relative;
      z-index: 10;
      width: 100%;
    }

    #sidebar-container {
      position: relative;
      z-index: 5;
      grid-area: sidebar;
    }

    #editor-container {
      grid-area: editor;
      width: 100%;
      display: flex;
      justify-content: center;
    }
  }
</style>
