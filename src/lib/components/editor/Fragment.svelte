<script>
  import { editor } from "../../stores/EditorStore";
  import { convertMarkdown } from '../../utils/utils.editor.js'
  export let key
  export let level
  export let content
  export let active
  export let fragments
  let el

  let fragContent
  let actionKeys = [
    "ArrowUp",
    "ArrowDown"
  ]

  let getFragContent = async() => {
    fragContent = active ? content : await convertMarkdown(content)
    // set caret to end of input
    let sel = window.getSelection()
    let range = document.createRange()
    sel.removeAllRanges()
    range.selectNodeContents(document.getElementsByClassName("active")[0])
    range.collapse(false)
    sel.addRange(range)
    document.getElementsByClassName("active")[0]?.focus()
  }

  $: fragments, getFragContent()

  let handleKeydown = (e) => {
    console.log(e)
    if (actionKeys.includes(e.key))
      e.preventDefault()
    
    // handle action keys 
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      let newFrag = {
        key: key + 1,
        level: level,
        content: '',
        active: true
      }
      fragments.splice(key + 1, 0, newFrag)
      fragments[key].active = false
      let i = key + 2 
      for (i; i < fragments.length; i++) {
        fragments[i].key++
      }
      fragments = fragments
    }
    if (e.key === "Backspace") {
      if (content === "") {
        e.preventDefault()
        fragments.splice(key, 1)
        fragments[key-1].active = true
        let i = key
        for (i; i < fragments.length; i++) {
          fragments[i].key--
        }
      } 
      else if (window.getSelection().getRangeAt(0).endOffset === 1) {
        fragments[key-1].content += ' ' + fragments[key].content 
        fragments.splice(key, 1)
        let i = key
        for (i; i < fragments.length; i++) {
          fragments[i].key--
        }
        fragments[key-1].active = true
      }
      fragments = fragments
    }
    if (e.key === "ArrowUp") {
      fragments[key-1].active = true
      fragments[key].active = false
      fragments = fragments 
    }
    if (e.key === "ArrowDown") {
      fragments[key+1].active = true
      fragments[key].active = false
      fragments = fragments
    }
  }

</script>

<div id="frag-{key}" class="fragment">
  <div class="handle" style={`margin-left:calc(1em*${level})`}>•</div>
  <div
    class={`content ${active ? 'active' : ''}`}
    on:focus={() => {fragments[key].active = true;fragments = fragments}}
    on:focusout={() => {fragments[key].active = false;fragments = fragments}}
    on:input={(e) => fragments[key].content = e.target.innerText}
    on:keydown={handleKeydown}
    contenteditable
  >
    {@html fragContent}
  </div> 
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
