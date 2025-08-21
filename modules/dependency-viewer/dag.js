import * as d3 from "d3";

export function initializeGraph(nodes, links, width, height) {
  const _svg = document.getElementById("canvas");
  if (_svg !== null) {
    document.removeChild(_svg);
  }

  const svg = d3.select("#visualization")
    .append("svg")
    .attr("id", "canvas")
    .attr("width", width)
    .attr("height", height)
    .attr("viewBox", [0, 0, width, height]);

  const g = svg.append("g");

  const zoom = d3.zoom()
    .scaleExtent([0.1, 10])
    .on("zoom", (event) => {
      g.attr("transform", event.transform);
    });

  svg.call(zoom);

  svg.call(zoom.transform, d3.zoomIdentity.translate(width / 2, height / 2));

  const getPackageName = (id) => id.split('.')[0];

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

  nodes.forEach(node => {
    node.package = getPackageName(node.id);
    node.indegree = indegree[node.id] || 0;
  });

  links.forEach(link => {
    const sourceId = typeof link.source === 'object' ? link.source.id : link.source;
    link.package = getPackageName(sourceId);
  });

  const packages = [...new Set(nodes.map(node => node.package))];

  function generateGoldenRatioColors(n) {
    const goldenRatio = (1 + Math.sqrt(5)) / 2;
    return d3.range(n).map(i =>
      d3.interpolateRainbow((i / goldenRatio) % 1)
    );
  }

  const colorScale = d3.scaleOrdinal()
    .domain(packages)
    .range(generateGoldenRatioColors(packages.length));

  let nodeSizeScale = 2;
  const getNodeRadius = (d) => {
    const baseSize = 6;
    const sizeMultiplier = Math.max(1, Math.sqrt(d.indegree + 1));
    return baseSize * sizeMultiplier * nodeSizeScale / 2;
  };

  const simulation = d3.forceSimulation(nodes)
    .force("link", d3.forceLink(links).id(d => d.id).distance(600))
    .force("charge", d3.forceManyBody().strength(-30))
    .force("center", d3.forceCenter(width / 2, height / 2))
    .force("collide", d3.forceCollide().radius(d => getNodeRadius(d) * 2))
    .force("x", d3.forceX(width / 2).strength(0.05))
    .force("y", d3.forceY(height / 2).strength(0.05));

  const defs = svg.append("defs");

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

  node.append("title")
    .text(d => `${d.name} (${d.id})\nPackage: ${d.package}\nIncoming dependencies: ${d.indegree}`);

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

  const packageFilter = d3.select("#packageCheckboxes");

  const enabledPackages = new Set(packages);

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

  function updateVisibility() {
    node.classed("filtered", d => !enabledPackages.has(d.package));

    link.classed("filtered", d => {
      const sourceId = typeof d.source === 'object' ? d.source.id : d.source;
      const targetId = typeof d.target === 'object' ? d.target.id : d.target;
      const sourcePkg = getPackageName(sourceId);
      const targetPkg = getPackageName(targetId);

      return !enabledPackages.has(sourcePkg) || !enabledPackages.has(targetPkg);
    });
  }

  simulation.on("tick", () => {
    link.attr("d", d => {
      const sourceRadius = typeof d.source === 'object' ? getNodeRadius(d.source) : 8;
      const targetRadius = typeof d.target === 'object' ? getNodeRadius(d.target) : 8;

      const dx = d.target.x - d.source.x;
      const dy = d.target.y - d.source.y;
      const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;

      const length = Math.sqrt(dx * dx + dy * dy);
      if (length === 0) return "M0,0L0,0";

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

  d3.select("#nodeSizeSlider").on("input", function () {
    nodeSizeScale = +this.value;
    d3.select("#nodeSizeValue").text(nodeSizeScale);

    node.select("circle")
      .attr("r", d => getNodeRadius(d));

    node.select("text")
      .attr("dx", d => getNodeRadius(d) + 4);

    simulation.force("collide").radius(d => getNodeRadius(d) * 2);

    simulation.alpha(0.3).restart();
  });

  d3.select("#findCycles").on("click", highlightCycles);

  function highlightCycles() {
    link.classed("cycle", false);

    const visited = new Set();
    const recStack = new Set();
    const adjList = {};

    nodes.forEach(node => {
      adjList[node.id] = [];
    });

    links.forEach(link => {
      adjList[link.source.id || link.source].push({
        target: link.target.id || link.target,
        linkObj: link
      });
    });


    function findAllPaths() {
      const cycleLinks = new Set();

      nodes.forEach(source => {
        if (node.filter(d => d.id === source.id).classed("filtered")) return;

        const paths = findPaths(source.id, source.id, new Set(), []);
        if (paths.length > 0) {
          paths.forEach(path => {
            for (let i = 0; i < path.length - 1; i++) {
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
        if (node.filter(d => d.id === start).classed("filtered")) return [];

        if (path.length > 0 && start === target) {
          return [path.slice()];
        }

        if (visited.has(start)) {
          return [];
        }

        const newVisited = new Set(visited);
        newVisited.add(start);
        const newPath = path.concat(start);

        let allPaths = [];
        (adjList[start] || []).forEach(neighbor => {
          if (node.filter(d => d.id === neighbor.target).classed("filtered")) return;

          const paths = findPaths(neighbor.target, target, newVisited, newPath);
          allPaths = allPaths.concat(paths);
        });

        return allPaths;
      }
    }

    findAllPaths();
  }

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

  setTimeout(highlightCycles, 1000);

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
