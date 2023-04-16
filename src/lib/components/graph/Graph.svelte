<script>
  import Sigma from 'sigma'
  import Graph from 'graphology'
  import circular from 'graphology-layout/circular'
  import random from 'graphology-layout/random'
  import forceAtlas2 from 'graphology-layout-forceatlas2'
  import ForceSupervisor from "graphology-layout-force/worker"
  import { onMount } from 'svelte';
  import { getNodes, getReferences, indexNodes } from '../../utils/utils.database'
  import { createNode } from '../../utils/utils.network';
  import { isDateFormat } from '../../utils/utils.editor';
  import { editor } from '../../stores/EditorStore';
  import { user } from '../../stores/UserStore'

  let draggedNode = null
  let isDragging = false
  let startClick = null 
  let endClick = null

  onMount(async () => {
    // index nodes
    indexNodes($user.config.network_config.location + '/' + $user.config.network_config.name)
    // get nodes and references
    let nodes = await getNodes()
    let references = await getReferences()
    let graphContainer = document.getElementById("sigmaContainer")

    const graph = new Graph()

    nodes.forEach(node => {
      graph.addNode(node, { label: node, nodeType: isDateFormat(node) ? "journal" : "node" })
    })

    references.forEach(ref => {
      graph.addEdge(ref.source_node, ref.target_node)
    })

    const COLORS = {
      journal: "#106ba3",
      node: "#fc4503"
    }

    graph.forEachNode((node, attributes) => {
      graph.setNodeAttribute(node, "color", COLORS[attributes.nodeType])
    })
    const degrees = graph.nodes().map((node) => graph.degree(node))
    const minDegree = Math.min(...degrees)
    const maxDegree = Math.max(...degrees)
    const minSize = 8,
      maxSize = 20
    graph.forEachNode((node) =>{
      const degree = graph.degree(node)
      graph.setNodeAttribute(
        node,
        "size",
        minSize + ((degree - minDegree) / (maxDegree - minDegree)) * (maxSize - minSize),
      )
    })

    // circular.assign(graph)
    random.assign(graph)

    const layout = new ForceSupervisor(graph, { 
      isNodeFixed: (_, attr) => attr.highlighted,
      settings: {
        gravity: 0.0001,
        inertia: 0
      }
    })
    layout.start()

    const renderer = new Sigma(graph, graphContainer)

    // navigate to node on click
    let handleOpenNode = (node) => {
      console.log(node)
      if (isDateFormat(node)) {
        createNode($user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/', node)
      } else {
        createNode($user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/', node)
      }

      let nodePath
      if (isDateFormat(node)) {
        nodePath = $user.config.network_config.location + '/' + $user.config.network_config.name + '/journal/' + node + '.md'
      } else {
        nodePath = $user.config.network_config.location + '/' + $user.config.network_config.name + '/nodes/' + node + '.md'
      }

      $editor = {
        ...editor,
        activeNode: node,
        nodePath: nodePath,
        isJournal: isDateFormat(node),
        showJournal: false,
        showEditor: true,
      }
    }

    // allow dragging nodes
    renderer.on("downNode", (e) => {
      isDragging = true
      draggedNode = e.node
      startClick = +new Date()
      graph.setNodeAttribute(draggedNode, "highlighted", true)
    })

    renderer.getMouseCaptor().on("mousemovebody", (e) => {
      if (!isDragging || !draggedNode) return

      const pos = renderer.viewportToGraph(e)

      graph.setNodeAttribute(draggedNode, "x", pos.x)
      graph.setNodeAttribute(draggedNode, "y", pos.y)

      e.preventSigmaDefault()
      e.original.preventDefault()
      e.original.stopPropagation()
    })

    renderer.getMouseCaptor().on("mouseup", () => {
      endClick = +new Date()
      let clickTime = endClick - startClick
      if (clickTime < 200) {
        handleOpenNode(draggedNode)
      } else {
        if (draggedNode)
          graph.removeNodeAttribute(draggedNode, "highlighted")
      }
      isDragging = false
      draggedNode = null
    })

    renderer.getMouseCaptor().on("mousedown", () => {
      if (!renderer.getCustomBBox()) renderer.setCustomBBox(renderer.getBBox())
    })
  })
</script>

<div id="sigmaContainer">

</div>

<style lang="scss">
  #sigmaContainer {
    width: 100vw;
    height: 100vh;
  }
</style>
