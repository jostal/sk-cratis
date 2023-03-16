<script>
  import Searchbar from './searchbar/Searchbar.svelte'
</script>

<nav>
  <div id="left-nav">

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
    width: 100%;

    #search-wrapper {
      grid-area: search;
      display: flex;
      align-items: center;
      justify-content: center;
    }

    #left-nav {
      grid-area: left;
      display: flex;
      justify-content: center;
      align-items: center;
    }

    #right-nav {
      grid-area: right;
      dislay: flex;
      justify-content: center;
      align-items: center;
    }
  }
</style>
