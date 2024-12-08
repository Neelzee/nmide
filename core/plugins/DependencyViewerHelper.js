window.plugins.set(
  "DependencyViewerHelper",
  {
    init: () => [],
    update: (_, __) => [],
    view: (_) => {
      return {
        kind: "frag",
        kids: [],
        text: null,
        attrs: [],
      };
    },
    render: (graph) => {
      if (graph === "") return;
      const width = 1280
      const height = 720

      const data = JSON.parse(graph);

      async function populateGraph() {
        const svg = d3.select("#graph svg")

        const links = data.links.map(d => ({ ...d }))
        const nodes = data.nodes.map(d => ({ ...d }))

        const simulation = d3.forceSimulation(nodes)
          .force("link", d3.forceLink(links).id(d => d.id))
          .force("charge", d3.forceManyBody())
          .force("center", d3.forceCenter(width / 2, height / 2))
          .force("x", d3.forceX(width / 2).strength(0.075))
          .force("y", d3.forceY(height / 2).strength(0.15))
          .on("tick", ticked)

        const link = svg.append("g")
          .attr("stroke", "#999")
          .attr("stroke-opacity", 0.6)
          .selectAll()
          .data(links)
          .join("line")
          .attr("stroke-width", d => (3 - (d.dist_sqrd / 500) * 2))

        const node = svg.append("g")
          .attr("stroke", "#fff")
          .attr("stroke-width", 1.5)
          .selectAll()
          .data(nodes)
          .join("circle")
          .attr("r", 5)
          .attr("fill", "blue");

        node.append("title")
          .text(d => d.id)

        node.call(d3.drag()
          .on("start", dragstarted)
          .on("drag", dragged)
          .on("end", dragended));

        function ticked() {
          link
            .attr("x1", d => d.source.x)
            .attr("y1", d => d.source.y)
            .attr("x2", d => d.target.x)
            .attr("y2", d => d.target.y);

          node
            .attr("cx", d => d.x)
            .attr("cy", d => d.y);
        }

        function dragstarted(event) {
          if (!event.active) simulation.alphaTarget(0.3).restart();
          event.subject.fx = event.subject.x;
          event.subject.fy = event.subject.y;
        }

        function dragged(event) {
          event.subject.fx = event.x;
          event.subject.fy = event.y;
        }

        function dragended(event) {
          if (!event.active) simulation.alphaTarget(0);
          event.subject.fx = null;
          event.subject.fy = null;
        }
      }


      async function initGraph() {
        d3.select("#graph")
          .append("svg")
          .attr("width", width)
          .attr("height", height)
          .attr("viewBox", [0, 0, width, height])
          .attr("style", "max-width: 100%; height: auto; height: intrinsic")

        populateGraph();
      }

      initGraph();
    },
  }
);
