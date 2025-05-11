import * as d3 from "d3";

export function initializeGraph(nodes, links) {
  // Get the container dimensions
  const container = document.getElementById("content");
  if (container == null) {
    console.error("Could not find element with id: `content`");
    return undefined;
  }
  const width = container.clientWidth;
  const height = container.clientHeight;

  // Create SVG with zoom capabilities
  const svg = d3.select("#visualization")
    .append("svg")
    .attr("id", "canvas")
    .attr("width", width)
    .attr("height", height)
    .attr("viewBox", [0, 0, width, height]);

  // Add zoom behavior
  const g = svg.append("g");

  const zoom = d3.zoom()
    .scaleExtent([0.1, 10])
    .on("zoom", (event) => {
      g.attr("transform", event.transform);
    });

  svg.call(zoom);

  // Center the view initially
  svg.call(zoom.transform, d3.zoomIdentity.translate(width / 2, height / 2));

  // Process nodes and links
  // Extract package names and create a color scale
  const getPackageName = (id) => id.split('.')[0];

  // Calculate indegree (number of incoming links) for each node
  const calculateIndegree = () => {
    const indegree = {};
    nodes.forEach(node => {
      indegree[node.id] = 0;
    });

    links.forEach(link => {
      const targetId = typeof link.target === 'object' ? link.target.id : link.target;
      indegree[targetId] = (indegree[targetId] || 0) + 1;
    });

    return indegree;
  };

  const indegree = calculateIndegree();

  // Update nodes with package and indegree information
  nodes.forEach(node => {
    node.package = getPackageName(node.id);
    node.indegree = indegree[node.id] || 0;
  });

  // Update links with package information
  links.forEach(link => {
    const sourceId = typeof link.source === 'object' ? link.source.id : link.source;
    link.package = getPackageName(sourceId);
  });

  // Get unique packages
  const packages = [...new Set(nodes.map(node => node.package))];

  // Create color scale for packages
  const colorScale = d3.scaleOrdinal()
    .domain(packages)
    .range(d3.schemeCategory10);

  // Node size scale based on indegree
  let nodeSizeScale = 2; // Default scale factor
  const getNodeRadius = (d) => {
    const baseSize = 6;
    const sizeMultiplier = Math.max(1, Math.sqrt(d.indegree + 1));
    return baseSize * sizeMultiplier * nodeSizeScale / 2;
  };

  // Create force simulation
  const simulation = d3.forceSimulation(nodes)
    .force("link", d3.forceLink(links).id(d => d.id).distance(100))
    .force("charge", d3.forceManyBody().strength(-30))
    .force("center", d3.forceCenter(width / 2, height / 2))
    .force("collide", d3.forceCollide().radius(d => getNodeRadius(d) * 2))
    .force("x", d3.forceX(width / 2).strength(0.05))
    .force("y", d3.forceY(height / 2).strength(0.05));

  // Create arrow markers for directed edges with colors for each package
  const defs = svg.append("defs");

  // Create a default arrow marker
  defs.append("marker")
    .attr("id", "arrowhead")
    .attr("viewBox", "0 -5 10 10")
    .attr("refX", 15)
    .attr("refY", 0)
    .attr("orient", "auto")
    .attr("markerWidth", 6)
    .attr("markerHeight", 6)
    .append("path")
    .attr("d", "M0,-5L10,0L0,5")
    .attr("fill", "#999");

  // Create colored arrow markers for each package
  packages.forEach(pkg => {
    defs.append("marker")
      .attr("id", `arrowhead-${pkg}`)
      .attr("viewBox", "0 -5 10 10")
      .attr("refX", 15)
      .attr("refY", 0)
      .attr("orient", "auto")
      .attr("markerWidth", 6)
      .attr("markerHeight", 6)
      .append("path")
      .attr("d", "M0,-5L10,0L0,5")
      .attr("fill", colorScale(pkg));
  });

  // Create links and nodes
  const link = g.append("g")
    .selectAll("path")
    .data(links)
    .enter()
    .append("path")
    .attr("class", "link")
    .style("stroke", d => {
      const pkg = typeof d.source === 'object' ? d.source.package : getPackageName(d.source);
      return colorScale(pkg);
    })
    .attr("marker-end", d => {
      const pkg = typeof d.source === 'object' ? d.source.package : getPackageName(d.source);
      return `url(#arrowhead-${pkg})`;
    });

  const node = g.append("g")
    .selectAll(".node")
    .data(nodes)
    .enter()
    .append("g")
    .attr("class", "node")
    .call(d3.drag()
      .on("start", dragstarted)
      .on("drag", dragged)
      .on("end", dragended));

  const circles = node.append("circle")
    .attr("r", d => getNodeRadius(d))
    .style("fill", d => d3.color(colorScale(d.package)).brighter(0.5));

  node.append("text")
    .attr("dx", d => getNodeRadius(d) + 4)
    .attr("dy", ".35em")
    .text(d => d.name);

  // Add titles for tooltips with additional information
  node.append("title")
    .text(d => `${d.name} (${d.id})\nPackage: ${d.package}\nIncoming dependencies: ${d.indegree}`);

  // Create a legend for packages
  const legend = d3.select("#packageLegend");

  packages.forEach(pkg => {
    const legendItem = legend.append("div")
      .attr("class", "legend-item");

    legendItem.append("div")
      .attr("class", "legend-color")
      .style("background-color", colorScale(pkg));

    legendItem.append("span")
      .text(pkg);
  });

  // Create package filter checkboxes
  const packageFilter = d3.select("#packageCheckboxes");

  // Keep track of enabled packages
  const enabledPackages = new Set(packages);

  // Create a checkbox for each package
  packages.forEach(pkg => {
    const checkboxDiv = packageFilter.append("div")
      .attr("class", "package-checkbox");

    const checkbox = checkboxDiv.append("input")
      .attr("type", "checkbox")
      .attr("id", `checkbox-${pkg}`)
      .attr("checked", true)
      .on("change", function () {
        if (this.checked) {
          enabledPackages.add(pkg);
        } else {
          enabledPackages.delete(pkg);
        }
        updateVisibility();
      });

    const label = checkboxDiv.append("label")
      .attr("for", `checkbox-${pkg}`);

    label.append("span")
      .attr("class", "package-color")
      .style("background-color", colorScale(pkg));

    label.append("span")
      .text(pkg);
  });

  // Select/deselect all buttons
  d3.select("#selectAll").on("click", () => {
    packages.forEach(pkg => {
      d3.select(`#checkbox-${pkg}`).property("checked", true);
      enabledPackages.add(pkg);
    });
    updateVisibility();
  });

  d3.select("#deselectAll").on("click", () => {
    packages.forEach(pkg => {
      d3.select(`#checkbox-${pkg}`).property("checked", false);
      enabledPackages.delete(pkg);
    });
    updateVisibility();
  });

  // Function to update visibility based on enabled packages
  function updateVisibility() {
    // Update nodes visibility
    node.classed("filtered", d => !enabledPackages.has(d.package));

    // Update links visibility
    link.classed("filtered", d => {
      const sourceId = typeof d.source === 'object' ? d.source.id : d.source;
      const targetId = typeof d.target === 'object' ? d.target.id : d.target;
      const sourcePkg = getPackageName(sourceId);
      const targetPkg = getPackageName(targetId);

      return !enabledPackages.has(sourcePkg) || !enabledPackages.has(targetPkg);
    });
  }

  // Update positions on tick
  simulation.on("tick", () => {
    link.attr("d", d => {
      // Adjust path for larger nodes
      const sourceRadius = typeof d.source === 'object' ? getNodeRadius(d.source) : 8;
      const targetRadius = typeof d.target === 'object' ? getNodeRadius(d.target) : 8;

      const dx = d.target.x - d.source.x;
      const dy = d.target.y - d.source.y;
      const dr = Math.sqrt(dx * dx + dy * dy) * 1.5; // Controls the curve

      // Calculate the total length
      const length = Math.sqrt(dx * dx + dy * dy);
      if (length === 0) return "M0,0L0,0"; // Handle edge case

      // Calculate the points with offset for the node radius
      const offsetRatio = sourceRadius / length;
      const startX = d.source.x + dx * offsetRatio;
      const startY = d.source.y + dy * offsetRatio;

      const endOffsetRatio = targetRadius / length;
      const endX = d.target.x - dx * endOffsetRatio;
      const endY = d.target.y - dy * endOffsetRatio;

      return `M${startX},${startY}A${dr},${dr} 0 0,1 ${endX},${endY}`;
    });

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

  // Zoom controls
  d3.select("#zoomIn").on("click", () => {
    svg.transition().duration(300).call(zoom.scaleBy, 1.5);
  });

  d3.select("#zoomOut").on("click", () => {
    svg.transition().duration(300).call(zoom.scaleBy, 0.75);
  });

  d3.select("#resetZoom").on("click", () => {
    svg.transition().duration(300).call(
      zoom.transform,
      d3.zoomIdentity.translate(width / 2, height / 2)
    );
  });

  // Force adjustment sliders
  d3.select("#chargeSlider").on("input", function () {
    const value = +this.value;
    d3.select("#chargeValue").text(value);
    simulation.force("charge").strength(value);
    simulation.alpha(0.3).restart();
  });

  d3.select("#linkDistanceSlider").on("input", function () {
    const value = +this.value;
    d3.select("#linkDistanceValue").text(value);
    simulation.force("link").distance(value);
    simulation.alpha(0.3).restart();
  });

  // Node size adjustment slider
  d3.select("#nodeSizeSlider").on("input", function () {
    nodeSizeScale = +this.value;
    d3.select("#nodeSizeValue").text(nodeSizeScale);

    // Update node circle sizes
    node.select("circle")
      .attr("r", d => getNodeRadius(d));

    // Update text position
    node.select("text")
      .attr("dx", d => getNodeRadius(d) + 4);

    // Update collision detection radius
    simulation.force("collide").radius(d => getNodeRadius(d) * 2);

    // Restart simulation
    simulation.alpha(0.3).restart();
  });

  // Function to detect and highlight cycles
  d3.select("#findCycles").on("click", highlightCycles);

  function highlightCycles() {
    // Reset all links to normal style
    link.classed("cycle", false);

    // Find cycles using depth-first search
    const visited = new Set();
    const recStack = new Set();
    const adjList = {};

    // Create adjacency list
    nodes.forEach(node => {
      adjList[node.id] = [];
    });

    links.forEach(link => {
      adjList[link.source.id || link.source].push({
        target: link.target.id || link.target,
        linkObj: link
      });
    });

    // Function to check if a node is part of a cycle
    // TODO: Implement
    function isCyclic(nodeId, parent) {
      return false;
      // Skip filtered nodes
      if (node.filter(d => d.id === nodeId).classed("filtered")) return false;

      // Mark the current node as visited and add to recursion stack
      visited.add(nodeId);
      recStack.add(nodeId);

      // Visit all neighbors
      for (const neighbor of adjList[nodeId]) {
        // Skip filtered neighbors
        if (node.filter(d => d.id === neighbor.target).classed("filtered")) continue;

        // If the neighbor is in recursion stack, we found a cycle
        if (recStack.has(neighbor.target)) {
          // Mark the link as part of a cycle
          link.each(function (d) {
            if ((d.source.id === nodeId && d.target.id === neighbor.target) ||
              (d.source === nodeId && d.target === neighbor.target)) {
              d3.select(this).classed("cycle", true);
            }
          });
          return true;
        }

        // If the neighbor hasn't been processed, process it
        if (!visited.has(neighbor.target)) {
          if (isCyclic(neighbor.target, nodeId)) {
            return true;
          }
        }
      }

      // Remove the node from recursion stack
      recStack.delete(nodeId);
      return false;
    }

    // Check all nodes
    nodes.forEach(node => {
      return;
      if (!visited.has(node.id) && !d3.select(`.node[data-id="${node.id}"]`).classed("filtered")) {
        isCyclic(node.id, null);
      }
    });

    // Alternative approach - find all paths between each node pair
    function findAllPaths() {
      const cycleLinks = new Set();

      nodes.forEach(source => {
        // Skip filtered nodes
        if (node.filter(d => d.id === source.id).classed("filtered")) return;

        // For each node, check if we can get back to itself
        const paths = findPaths(source.id, source.id, new Set(), []);
        if (paths.length > 0) {
          // We found cycles, mark the links
          paths.forEach(path => {
            for (let i = 0; i < path.length - 1; i++) {
              // Skip filtered nodes
              if (node.filter(d => d.id === path[i]).classed("filtered") ||
                node.filter(d => d.id === path[i + 1]).classed("filtered")) continue;

              link.each(function (d) {
                const linkSrc = d.source.id || d.source;
                const linkTgt = d.target.id || d.target;
                if (linkSrc === path[i] && linkTgt === path[i + 1]) {
                  d3.select(this).classed("cycle", true);
                }
              });
            }
          });
        }
      });

      function findPaths(start, target, visited, path) {
        // Skip filtered nodes
        if (node.filter(d => d.id === start).classed("filtered")) return [];

        if (path.length > 0 && start === target) {
          return [path.slice()]; // Found a cycle
        }

        if (visited.has(start)) {
          return []; // Already visited
        }

        const newVisited = new Set(visited);
        newVisited.add(start);
        const newPath = path.concat(start);

        let allPaths = [];
        (adjList[start] || []).forEach(neighbor => {
          // Skip filtered nodes
          if (node.filter(d => d.id === neighbor.target).classed("filtered")) return;

          const paths = findPaths(neighbor.target, target, newVisited, newPath);
          allPaths = allPaths.concat(paths);
        });

        return allPaths;
      }
    }

    // Call the alternative approach for better cycle detection
    findAllPaths();
  }

  // Update package checkboxes when clicking on legend items
  d3.selectAll(".legend-item").on("click", function () {
    const pkgText = d3.select(this).select("span").text();
    const checkbox = d3.select(`#checkbox-${pkgText}`);
    const newChecked = !checkbox.property("checked");

    checkbox.property("checked", newChecked);

    if (newChecked) {
      enabledPackages.add(pkgText);
    } else {
      enabledPackages.delete(pkgText);
    }

    updateVisibility();
  });

  // Call highlightCycles on load to show cycles immediately
  setTimeout(highlightCycles, 1000);

  // Return the important objects for potential updates
  return {
    svg,
    simulation,
    zoom,
    nodes,
    links,
    width,
    height
  };
}