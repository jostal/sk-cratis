<script>
  import { indexNodes } from '../../utils/utils.database';
  import { editor } from '../../stores/EditorStore';
  import { user } from '../../stores/UserStore';
  import Searchbar from './searchbar/Searchbar.svelte'

  let handleShowJournal = () => {
    $editor = {
      activeNode: getDate(),
      nodePath: $user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/' + getDate() + '.md',
      activeFragment: 0,
      isJournal: true,
      showJournal: true 
    }
  }

  let getDate = () => {
    let today = new Date()
    let dd = String(today.getDate()).padStart(2, '0')
    let mm = String(today.getMonth() + 1).padStart(2, '0')
    let yyyy = today.getFullYear()

    return yyyy + '-' + mm + '-' + dd
  }
</script>

<nav>
  <div id="left-nav">
    <button on:click={handleShowJournal} title="Journal"><img src="calendar.svg" alt="journals"/></button>
    <button on:click={() => indexNodes($user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/')} title="Re-index network"><img src="network.svg" alt="Re-index network" /></button>
  </div>
  <div id="search-wrapper">
    <Searchbar />
  </div>
  <div id="right-nav">

  </div>
</nav>

<style lang="scss">
  nav {
    height: var(--nav-height);
    background-color: var(--nav-color);
    display: grid;
    grid-template-areas: 
      "left search right";
    grid-template-columns: 33% 33% 33%;
    width: 100%;
    position: relative;
    padding: 1em;
    z-index: 10;

    #search-wrapper {
      position: absolute;
      grid-area: search;
      display: flex;
      align-items: center;
      justify-content: center;
      justify-self: center;
      align-self: center;
      width: 100%;
    }

    #left-nav {
      grid-area: left;
      display: flex;
      align-items: center;
      gap: 1em;

      button {
        background: transparent;
        border: none;
        border-radius: 0.3em;
        padding: 0.5em;
        width: 3.5em;
        height: 3.5em;

        &:hover {
          background-color: var(--select-bg);
        }

        img {
          width: 100%;
        }
      }

    }

    #right-nav {
      grid-area: right;
      display: flex;
      justify-content: center;
      align-items: center;
    }
  }
</style>
