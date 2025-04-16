
const module = "DependencyViewerHelper";
window.__nmideConfig__.modules.set(
  module,
  {
    name: module,
    init: async (core) => {
      const script = document.createElement("script");
      script.type = "text/javascript";
      script.src = "https://cdn.jsdelivr.net/npm/d3@7"
      document.head.appendChild(script);
      await core.registerHandler(module, "graph", null)
        .catch((err) => console.error("error from module: ", err));
      return { state: "noOp", ui: [{ add: [null, null, { svg: { kids: [], attrs: [{ id: "graph" }] } }] }, "noOp", "noOp"] };
    },
    handler: (event, __) => {
      if (event.event === "graph") {

// Process the data into nodes and links
        function processData(data) {
          const nodes = [];
          const links = [];
          const nodeMap = new Map();

          // First pass: create all nodes
          data.list.forEach(item => {
            const moduleName = item.obj.name.str;
            if (!nodeMap.has(moduleName)) {
              const node = { id: moduleName, name: moduleName };
              nodes.push(node);
              nodeMap.set(moduleName, node);
            }

            // Process dependencies
            if (item.obj.dependencies && item.obj.dependencies.list) {
              item.obj.dependencies.list.forEach(dep => {
                const depName = dep.str;
                if (!nodeMap.has(depName)) {
                  const depNode = { id: depName, name: depName };
                  nodes.push(depNode);
                  nodeMap.set(depName, depNode);
                }
              });
            }
          });

          // Second pass: create links
          data.list.forEach(item => {
            const sourceName = item.obj.name.str;
            if (item.obj.dependencies && item.obj.dependencies.list) {
              item.obj.dependencies.list.forEach(dep => {
                const targetName = dep.str;
                links.push({
                  source: sourceName,
                  target: targetName
                });
              });
            }
          });

          return { nodes, links };
        }

// Render the graph
        function renderGraph(data) {
          const { nodes, links } = processData(data);
          const svg = d3.select("#graph");
          const width = svg.node().getBoundingClientRect().width;
          const height = svg.node().getBoundingClientRect().height;

          // Clear previous content
          svg.selectAll("*").remove();

          // Create a group for zoom/pan
          const g = svg.append("g");

          // Set up zoom behavior
          const zoom = d3.zoom()
            .scaleExtent([0.1, 5])
            .on("zoom", (event) => {
              g.attr("transform", event.transform);
            });

          svg.call(zoom);

          // Create a simulation
          const simulation = d3.forceSimulation(nodes)
            .force("link", d3.forceLink(links).id(d => d.id).distance(100))
            .force("charge", d3.forceManyBody().strength(-300))
            .force("center", d3.forceCenter(width / 2, height / 2))
            .force("collision", d3.forceCollide().radius(60));

          // Create arrows
          g.append("defs").selectAll("marker")
            .data(["end"])
            .enter().append("marker")
            .attr("id", d => d)
            .attr("viewBox", "0 -5 10 10")
            .attr("refX", 25)
            .attr("refY", 0)
            .attr("markerWidth", 6)
            .attr("markerHeight", 6)
            .attr("orient", "auto")
            .append("path")
            .attr("d", "M0,-5L10,0L0,5")
            .attr("fill", "#999");

          // Draw links
          const link = g.append("g")
            .selectAll("line")
            .data(links)
            .enter().append("line")
            .attr("stroke", "#999")
            .attr("stroke-width", 2)
            .attr("marker-end", "url(#end)");

          // Create node groups
          const node = g.append("g")
            .selectAll("g")
            .data(nodes)
            .enter().append("g")
            .call(d3.drag()
              .on("start", dragstarted)
              .on("drag", dragged)
              .on("end", dragended));

          // Add circles to nodes
          node.append("circle")
            .attr("r", 20)
            .attr("fill", "#4285f4")
            .attr("stroke", "#3367d6")
            .attr("stroke-width", 2);

          // Add text to nodes
          node.append("text")
            .attr("dy", 4)
            .attr("text-anchor", "middle")
            .text(d => d.name)
            .style("fill", "white")
            .style("font-family", "Arial")
            .style("font-size", "12px")
            .style("pointer-events", "none");

          // Add node labels
          const labels = node.append("text")
            .attr("dy", 35)
            .attr("text-anchor", "middle")
            .text(d => d.name)
            .style("fill", "#333")
            .style("font-family", "Arial")
            .style("font-size", "10px")
            .style("pointer-events", "none");

          // Update positions on simulation tick
          simulation.on("tick", () => {
            link
              .attr("x1", d => d.source.x)
              .attr("y1", d => d.source.y)
              .attr("x2", d => d.target.x)
              .attr("y2", d => d.target.y);

            node.attr("transform", d => `translate(${d.x},${d.y})`);
          });

          // Drag functions
          function dragstarted(event, d) {
            if (!event.active) simulation.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
          }

          function dragged(event, d) {
            d.fx = event.x;
            d.fy = event.y;
          }

          function dragended(event, d) {
            if (!event.active) simulation.alphaTarget(0);
            d.fx = null;
            d.fy = null;
          }
        }
        renderGraph(event.args);
      }

      return { state: "noOp", ui: ["noOp", "noOp", "noOp"] };
    },
  }
);
