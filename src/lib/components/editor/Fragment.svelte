<script>
  import { editor } from "../../stores/EditorStore";
  export let key
  export let level
  export let content
  export let html
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
  <div class="handle" style={`margin-left:calc(1em*${level})`}>•</div>
  {#if $editor.activeFragment === key}
    <div 
      class="content active"
      autofocus
      contenteditable
    >
      {content}
    </div>
  {:else}
    <div
      class="content"
      contenteditable
      on:focus={() => $editor.activeFragment = key}
    >
      {@html html} 
    </div>
  {/if}
</div>

<style lang="scss">
  .fragment {
    display: inline-grid;
    grid-template-areas: "handle content";

    .handle {
      grid-area: handle;
      align-self: center;
      padding: 0.1em;
      margin-right: 0.5em;
    }

    .content {
      grid-area: content;
      display: inline;
    }
  }
  :global {
    .content > p {
      display: inline;
      margin: 0;
    }
  }
</style>
