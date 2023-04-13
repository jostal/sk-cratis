<script>
  import { getSourceContent } from '../../utils/utils.database.js' 
  import { parseContent } from '../../utils/utils.editor.js';
  import { user } from '../../stores/UserStore.js';
  import { editor } from '../../stores/EditorStore.js';
  export let sourceNode
  export let targetNode
  let refFragments = []
  let filteredFrags = []

  let getRefFragments = async () => {
    let sourceContent = await getSourceContent(sourceNode, $user.config.network_config.location + "/" + $user.config.network_config.name)
    let lines = sourceContent.split(/\r?\n/).filter(l => l !== "")
    refFragments = new Array(lines.length)

    for (let i=0; i<lines.length; i++) {
      let parsedContent = await parseContent(lines[i])
      refFragments[i] = {
        key: i,
        level: parsedContent.level,
        content: parsedContent.content,
        active: false 
      }
      refFragments = refFragments
    }
    console.log(refFragments)

    // remove non-referencing fragments
    filteredFrags = filterFragments(refFragments)   
  }

  let filterFragments = (fragments) => {
    let filteredFrags = []
    let refLevel = 0
    let refKey = -1
    for (let frag of fragments) {
      if (frag.level <= refLevel) {
        refKey = -1
      }

      if (refKey === -1) {
        let linkRegex = new RegExp(`(?<=\\[\\[)${targetNode}(?=\\]\\])`, "g")
        let tagRegex = new RegExp(`(?<=\\B#)${targetNode}(?=\\b)`, "g")

        let linkMatch = frag.content.match(linkRegex)
        let tagMatch = frag.content.match(tagRegex)

        if (linkMatch || tagMatch) {
          let match
          if (linkMatch) match = linkMatch
          if (tagMatch) match = tagMatch
          console.log(match)
          if (match[0] === targetNode) {
            filteredFrags.push(frag)
            refKey = frag.key
          }
        }
      } else {
        filteredFrags.push(frag)
      }
    }
    return filteredFrags
  }

  $: $editor, getRefFragments() 
  
</script>

<div id="refSource">
  <button class="sourceNode">{sourceNode}</button> 
  {#each filteredFrags as frag}
    <div
      class="fragContainer"
      style={`margin-left:calc(1em*${frag.level})`}
    >
      <div class="handle">•</div>
      <div class="content">{@html frag.content}</div>
    </div> 
  {/each}
</div>

<style lang="scss">
  #refSource {
    .sourceNode {

    }

    .fragContainer {
      display: inline-grid;
      grid-template-areas: "handle content";
      grid-template-columns: 1em auto;
      width: 100%;

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
  }
</style>
