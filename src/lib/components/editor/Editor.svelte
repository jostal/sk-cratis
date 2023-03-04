<script>
  import { editor, openNode } from "../../stores/EditorStore";
  let content;
  let lines = []

  $: $editor, getContent()

  let getContent = async () => {
    content = await openNode($editor.nodePath, $editor.activeNode).then(res => res.content)
    if (content)
      lines = content.split(/\r?\n/)
  }

  let handleFragInput = (e) => {
    if (e.key === "Enter") {
      lines.push("")
      lines = lines
    }
  }

  let init = (el) => {
    el.focus()
  }
</script>

<section id="editor">
  <h1>
    <input value={$editor.activeNode} />
  </h1>
  {#each lines as line, i}
    <div key={i} class="fragment">
      <div class="fragment-handle">
        •
      </div>
      <input 
        value={line} 
        on:keydown|preventDefault={(e) => handleFragInput(e)}
        use:init
      />
    </div>
  {/each}
</section>

<style lang="scss">
  #editor { 
    padding: 1em;
    background-color: var(--sidebar-color);
    margin: 20px 10% 20px 10%;

    .fragment {
      display: grid;
      grid-template-areas: 
        "handle fragment";
      grid-template-columns: 1em auto;

      .fragment-handle {
        grid-area: handle;
        cursor: pointer;
      }

      input {
        grid-area: fragment;
      }
    }
  }

  input {
    border: none;
    background-color: var(--sidebar-color);
    color: white;
  }

  h1 {
    input {
      font-size: inherit;
    }
  }
</style>
