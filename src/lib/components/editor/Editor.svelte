<script>
  import { tick } from "svelte";
  import { editor, openNode } from "../../stores/EditorStore";
  import { user } from "../../stores/UserStore"
  import { createNode, getJournalEntries } from "../../utils/utils.network.js"
  import Node from "./Node.svelte";
  let lines = []
  let content
  let currentDate
  let journal = []

  let checkIsJournal = () => {
    if ($editor.isJournal)
      openJournal()
  }

  let getContent = async () => {
    lines = []
    content = await openNode($editor.nodePath, $editor.activeNode).then(res => res.content)
    if (content) {
      lines = content.split(/\r?\n/).filter(l => l !== "")
    } else {
      lines = [""]
    }
    console.log(lines)
    lines = lines
  }

  $: $editor, checkIsJournal()
  $: $editor.activeNode, getContent()

  let getDate = () => {
    let today = new Date()
    let dd = String(today.getDate()).padStart(2, '0')
    let mm = String(today.getMonth() + 1).padStart(2, '0')
    let yyyy = today.getFullYear()

    currentDate = yyyy + '-' + mm + '-' + dd 
  }

  let openJournal = () => {
    getDate()
    createNode($user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/', currentDate);
    $editor.activeNode = currentDate
    $editor.nodePath = $user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/' + currentDate + '.md'
    $editor.isJournal = true

    renderJournal()
  }

  let renderJournal = async () => {
    journal = []
    let journalEntries = await getJournalEntries($user.config.network_config.location + '/' + $user.config.network_config.name + '/journal')
    journalEntries.reverse()
    
    for (let i = 0; i < journalEntries.length; i++) {
      let entryLines = []
      if (journalEntries[i].content) {
         entryLines = journalEntries[i].content.split(/\r?\n/).filter(l => l !== "")
      } else {
        entryLines = [""]
      }

      journal.push({
        date: journalEntries[i].date,
        content: entryLines
      })
    }
    journal = journal
  }

</script>

<section id="editor">
  {#if $editor.isJournal}
    {#each journal as entry}
      <Node bind:lines={entry.content} nodeName={entry.date} />
    {/each}
  {:else}
    <Node bind:lines={lines} nodeName={$editor.activeNode} /> 
  {/if}
</section>

<style lang="scss">
  #editor { 
    padding: 1em;
    background-color: var(--sidebar-color);
    margin: 20px 10% 20px 10%;
    height: calc(100% - 5em);
  }
</style>
