<script>
  import { editor } from '../../stores/EditorStore.js'
  import { user } from '../../stores/UserStore.js'

  let handleShowJournal = () => {
    $editor = {
      activeNode: getDate(),
      nodePath: $user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/' + getDate() + '.md', 
      activeFragment: 0,
      isJournal: true
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

<section id="sidebar">
  <button on:click={handleShowJournal}>Journal</button>
</section>

<style lang="scss">
  #sidebar {
    position: sticky;
    z-index: 5;
    left: 0;
    top: var(--nav-height);
    background-color: var(--sidebar-color);
    height: calc(100vh - var(--nav-height));
  }
</style>
