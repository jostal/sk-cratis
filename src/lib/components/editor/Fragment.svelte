<script>
  export let key
  export let content
  export let handleNewFragment
  export let handleDeleteFragment
  export let handleMergeFragments
  export let handleFocusFragment
  let el

  let handleKeydown = (e) => {
    console.log(e.key)
    if (e.key === "Enter") {
      e.preventDefault()
      handleNewFragment(key)
    }
    if (e.key === "Backspace") {
      if (el.value === "") {
        e.preventDefault()
        handleDeleteFragment(key)
      }
      else if (e.target.selectionStart === 0) {
        e.preventDefault()
        handleMergeFragments(key)
      }
    }
    if (e.key === "ArrowUp") {
      e.preventDefault()
      handleFocusFragment(key-1)
    }
    if (e.key === "ArrowDown") {
      e.preventDefault()
      handleFocusFragment(key+1)
    }
  }

</script>

<div id="frag-{key}" class="fragment">
  <input 
    value={content} 
    on:keydown={handleKeydown}
    bind:this={el} 
  />
</div>

<style lang="scss">
  .fragment {
    input {
      padding: 0.3em;
      width: 100%;
      background-color: var(--sidebar-color);
      border: none;
      color: inherit;
    }
  }
</style>
