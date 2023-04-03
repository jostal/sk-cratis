<script>
  import { searchNodes } from '../../../utils/utils.editor.js'
  import { createNode } from '../../../utils/utils.network'
  import { user } from '../../../stores/UserStore.js';
  import { editor } from '../../../stores/EditorStore.js'
  let searchVal = ""
  let searchResults = []
  let activeNode = 0
  let isOpening = false 

  $: searchVal, handleSearch()
  
  let handleSearch = async () => {
    if (searchVal !== "") {
      searchResults = await searchNodes(searchVal, $user.config.network_config.location + '/' + $user.config.network_config.name) 
      if (searchResults[0] !== searchVal) { 
        searchResults.splice(0, 0, searchVal)
      }
    }
  }

  let handleOpenNode = (node) => {
    isOpening = true
    createNode($user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/', searchResults[activeNode]);
    $editor.activeNode = node 
    $editor.nodePath = $user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/' + node + '.md'
    $editor.isJournal = false
    searchResults = []
    searchVal = ""
  }

  let handleKeyPress = (e) => {
    if (e.key === "ArrowUp") {
      e.preventDefault()
      if (activeNode > 0)
        activeNode--
    }
    if (e.key === "ArrowDown") {
      e.preventDefault()
      if (activeNode < searchResults.length - 1)
        activeNode++
    }
    if (e.key === "Enter") {
      handleOpenNode(searchResults[activeNode])
      isOpening = false
    }
  }

  let handleFocusOut = () => {
    searchResults = []
    activeNode = 0
  }
</script>

<section id="search">
  <input 
    value={searchVal}
    on:input={(e) => searchVal = e.target.value}
    on:keydown={(e) => handleKeyPress(e)}
    on:blur={() => isOpening ? null : handleFocusOut()}
  />
  <ul id="search-results" on:mouseover={() => isOpening = true} on:mouseleave={() => isOpening = false} on:focus|preventDefault>
    {#each searchResults as node, i}
      <li 
        key={node}
        on:click={(e) => handleOpenNode(e.target.attributes[0].nodeValue)}
        on:keydown|preventDefault
        class={activeNode === i ? 'active' : ''}
        on:mouseover={() => activeNode = i}
        on:focus|preventDefault
      >
        {node}
      </li>
    {/each}
  </ul>
</section>

<style lang="scss">
  #search {
    width: 100%;

    input {
      width: 100%;
      padding: 0.3em;
    }

    #search-results {
      margin: 0;
      padding: 0.3em;
      width: 100%;
      position: absolute;
      z-index: 10;
      list-style-type: none;
      background-color: var(--nav-color);

      li {
        margin: 0;
      }

      .active {
        background-color: #575B4F;
      }
    }
  }
</style>
