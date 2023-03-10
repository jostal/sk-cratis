<script>
  import { editor } from "../../stores/EditorStore";
  import { convertMarkdown, saveNode } from '../../utils/utils.editor.js'
  import { tick } from 'svelte'
  export let key
  export let level
  export let content
  export let active
  export let fragments
  export let dragging
  let hoverHandle
  let dragSrcEl

  let fragContent
  let actionKeys = [
    "ArrowUp",
    "ArrowDown",
    "Tab"
  ]

  let getFragContent = async() => {
    fragContent = active ? content : await convertMarkdown(content)
    document.getElementsByClassName("active")[0]?.focus()
  }

  $: fragments, getFragContent()
  $: fragments, saveNode(fragments, $editor.nodePath)

  let handleKeydown = async (e) => {
    if (actionKeys.includes(e.key))
      e.preventDefault()
    
    // handle action keys 
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault()
      let caretInfo = getCaretPos(document.getElementsByClassName('active')[0])
      if (caretInfo.position === caretInfo.length && caretInfo.length > 0) {
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
      } else {
        if (caretInfo.length > 0) {
          // create new fragment with content following caret
          let elContent = getCaretContent(caretInfo.position)
          let newFrag = {
            key: key + 1,
            level: level,
            content: elContent.afterCaret,
            active: true 
          }
          fragments.splice(key + 1, 0, newFrag)
          fragments[key].active = false 
          fragments[key].content = fragments[key].content.slice(0, caretInfo.position)
          let i = key + 2
          for (i; i < fragments.length; i++) {
            fragments[i].key++
          }
          fragments = fragments
          await tick()
          setCaretPos(0)
        }
      }
    }

    if (e.key === "Backspace") {
      let caretPos = getCaretPos(document.getElementsByClassName('active')[0])
      let prevContent = fragments[key-1].content 
      if (content === "") {
        e.preventDefault()
        fragments.splice(key, 1)
        fragments[key-1].active = true
        let i = key
        for (i; i < fragments.length; i++) {
          fragments[i].key--
        }
      } 
      else if (caretPos.position === 0) {
        fragments[key-1].content += ' ' + fragments[key].content 
        fragments.splice(key, 1)
        let i = key
        for (i; i < fragments.length; i++) {
          fragments[i].key--
        }
        fragments[key-1].active = true
      }
      fragments = fragments
      await tick()
      setCaretPos(prevContent.length + 1)
    }

    if (e.key === "ArrowUp") {
      fragments[key-1].active = true
      fragments[key].active = false
      fragments = fragments 
      await tick()
      setCaretPos(fragments[key-1].content.length)
    }
    if (e.key === "ArrowDown") {
      fragments[key+1].active = true
      fragments[key].active = false
      fragments = fragments
      await tick()
      setCaretPos(fragments[key+1].content.length)
    }

    if (e.key === "Tab") {
      if (!e.shiftKey) {
        if (fragments[key - 1].level >= fragments[key].level)
          fragments[key].level++
      } else {
        if (level > 0)
          fragments[key].level--
      }
      fragments = fragments
    }
  }

  let getCaretContent = (caretPos) => {
    let sel = window.getSelection()
    if (sel.rangeCount !== 0) {
      let content = window.getSelection().getRangeAt(0).endContainer.textContent
      let beforeCaret = content.substring(0, caretPos)
      let afterCaret = content.substring(caretPos, content.length)

      return {
        content: content,
        beforeCaret: beforeCaret,
        afterCaret: afterCaret
      }
    }
  }

  let getCaretPos = (el) => {
    let pos = 0
    let sel = window.getSelection()
    let range = window.getSelection().getRangeAt(0)
    let preCaretRange = range.cloneRange()
    preCaretRange.selectNodeContents(el)
    preCaretRange.setEnd(range.endContainer, range.endOffset)
    pos = preCaretRange.toString().length 
   
    return {
      position: pos,
      length: range.endContainer.textContent.length
    }
  }

  let setCaretPos = (index) => {
    let el = document.getElementsByClassName('active')[0]
    let sel = window.getSelection()
    let range = document.createRange()
    range.setStart(el.childNodes[0], index)
    range.collapse(true)
    sel.removeAllRanges()
    sel.addRange(range)
  }

  let handleInput = async (e) => {
    if (document.getElementsByClassName('active')[0]) {
      let caretPos = getCaretPos(document.getElementsByClassName('active')[0])
      fragments[key].content = e.target.textContent 
      fragments = fragments
      await tick()
      setCaretPos(caretPos.position)
    }
  }

  let handleClick = async (type, e) => {
    if (type === "focus") {
      fragments[key].active = true
      fragments = fragments
      let pos = getCaretPos(e.target)
      await tick()
      setCaretPos(pos.position)
    }
    if (type === "blur") {
      fragments[key].active = false
      fragments = fragments 
    }
  }

  let handleDragStart = (e) => {
    // save source element 
    dragSrcEl = e.target
    e.dataTransfer.setData('text/plain', e.target.getAttribute('key'))
    dragging = true
  }
  
  let handleDragEnter = (e) => {
  }

  let handleDragEnd = (e) => {
    dragging = false
  }

  let handleDragOver = (e) => {
    return false
  }

  let handleDrop = (e) => {
    // clone drop fragment and move down 1
    let targetKey = parseInt(e.currentTarget.getAttribute('key'))
    let oldKey = parseInt(e.dataTransfer.getData('text'))
    let cloneFrag = {
      key: targetKey + 1,
      level: fragments[targetKey].level,
      content: fragments[targetKey].content,
      active: false 
    }
    fragments.splice(targetKey + 1, 0, cloneFrag)
    let i = targetKey + 2
    for (i; i < fragments.length; i++) {
      fragments[i].key++
    }

    if (targetKey < oldKey) {
     // replace drop fragment with drag fragment
      let oldFrag = {
        key: targetKey,
        level: fragments[targetKey+1].level,
        content: fragments[oldKey+1].content,
        active: true
      }
      fragments[targetKey] = oldFrag
      // delete drag fragment
      fragments.splice(oldKey+1, 1)
      i = oldKey+1
      for (i; i < fragments.length; i++) {
        fragments[i].key--
      }
    } else {
      let dragFrag = {
        key: targetKey,
        level: fragments[targetKey+1].level,
        content: fragments[oldKey].content,
        active: true 
      }
      fragments[targetKey] = dragFrag
      fragments.splice(oldKey, 1)
      i = oldKey
      for (i; i < fragments.length; i++) {
        fragments[i].key--
      }
    }
    fragments = fragments
  }

</script>

<div 
  id="frag-{key}" 
  key={key}
  class={`fragment ${dragging ? 'dragging' : ''}`}
  on:dragover|preventDefault={handleDragOver}
  on:dragenter|preventDefault={handleDragEnter}
  on:drop={handleDrop}
>
  <div 
    class={`frag-drag ${hoverHandle ? 'draggable' : ''}`}
    key={key}
    on:dragstart={handleDragStart}
    on:dragend={handleDragEnd}
    draggable="true"
  >
    <div 
      class="handle" 
      style={`margin-left:calc(1em*${level})`}
      on:mouseenter={() => hoverHandle = true}
      on:mouseleave={() => hoverHandle = false}
    >???</div>
    <div
      class={`content ${active ? 'active' : ''}`}
      on:click={(e) => handleClick("focus", e)}
      on:blur={(e) => handleClick("blur", e)}
      on:input={handleInput}
      on:keydown={handleKeydown}
      contenteditable
    >
      {@html fragContent}
    </div> 
  </div>
 
</div>

<style lang="scss">
  .frag-drag {
    pointer-events: none;
    display: inline-grid;
    grid-template-areas: "handle content";
    grid-template-columns: 1em auto;
    width: 100%;

    .handle {
      grid-area: handle;
      align-self: center;
      padding: 0.1em;
      margin-right: 0.5em;
      pointer-events: auto;
    }

    .content {
      grid-area: content;
      display: inline;
      pointer-events: auto;
    }
  }
  :global {
    .content > p {
      display: inline;
      margin: 0;
    }
  }
  .dragging {
    
  }

  .draggable {
    cursor: move;
  }
</style>
