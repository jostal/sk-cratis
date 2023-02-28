<script>
  import { user, updateConfig } from './lib/stores/UserStore.js';
  import { open } from '@tauri-apps/api/dialog';
  import { documentDir } from '@tauri-apps/api/path';
  import { createNetwork } from './lib/utils/utils.network.js';
  import Sidebar from './lib/components/sidebar/Sidebar.svelte';
    import Editor from './lib/components/editor/Editor.svelte';

  let requestNetworkLocation = $user.config.network_config.location === "";
  let dir;

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
      <Sidebar />
      <Editor />
    </div>
  {/if}
</main>

<style>
</style>
