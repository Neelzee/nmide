export function initializeTree() {
  // Store global references to the SVG and root data
  let svg;
  let root;
  let treeLayout;
  let tooltip;
  let margin;
  let width;
  let height;

  // Initialize the visualization once
  function initialize() {
    // Set up dimensions
    margin = { top: 30, right: 90, bottom: 30, left: 90 };
    width = 1100 - margin.left - margin.right;
    height = 800 - margin.top - margin.bottom;

    // Create tooltip if it doesn't exist
    if (!tooltip) {
      tooltip = d3.select("body").select(".state-viz-tooltip");
      if (tooltip.empty()) {
        tooltip = d3.select("body").append("div")
          .attr("class", "state-viz-tooltip")
          .style("opacity", 0);
      }
    }

    // Clear existing SVG if it exists
    d3.select("#state-viz").selectAll("svg").remove();

    // Create the SVG container
    svg = d3.select("#state-viz").append("svg")
      .attr("width", width + margin.left + margin.right)
      .attr("height", height + margin.top + margin.bottom)
      .append("g")
      .attr("transform", `translate(${margin.left},${margin.top})`);

    // Create the tree layout
    treeLayout = d3.tree().size([height, width - 160]);
  }

  // Function to convert the JSON to a hierarchical structure suitable for D3
  function convertToHierarchy(data, key = "root") {
    let node = { name: key };

    if (data === null || data === "null") {
      node.name = `${key}: null`;
      node.type = "null";
      return node;
    }

    if (typeof data !== 'object') {
      node.name = `${key}: ${data}`;
      node.type = "primitive";
      return node;
    }

    // Process object types
    if (data.hasOwnProperty('obj')) {
      node.name = `${key} (object)`;
      node.type = "object";
      node.children = [];
      for (const [childKey, childValue] of Object.entries(data.obj)) {
        node.children.push(convertToHierarchy(childValue, childKey));
      }
    }
    // Process list types
    else if (data.hasOwnProperty('list')) {
      node.name = `${key} (array)`;
      node.type = "list";
      node.children = [];
      data.list.forEach((item, index) => {
        node.children.push(convertToHierarchy(item, `${key}[${index}]`));
      });
    }
    // Process primitive types
    else if (data.hasOwnProperty('int')) {
      node.name = `${key}: ${data.int} (int)`;
      node.type = "primitive";
    }
    else if (data.hasOwnProperty('float')) {
      node.name = `${key}: ${data.float} (float)`;
      node.type = "primitive";
    }
    else if (data.hasOwnProperty('str')) {
      node.name = `${key}: "${data.str}" (string)`;
      node.type = "primitive";
    }
    else if (data.hasOwnProperty('bool')) {
      node.name = `${key}: ${data.bool} (bool)`;
      node.type = "primitive";
    }
    // Process regular objects
    else {
      node.name = `${key} (object)`;
      node.type = "object";
      node.children = [];
      for (const [childKey, childValue] of Object.entries(data)) {
        node.children.push(convertToHierarchy(childValue, childKey));
      }
    }

    return node;
  }

  // Function to collapse nodes
  function collapse(d) {
    if (d.children) {
      d._children = d.children;
      d._children.forEach(collapse);
      d.children = null;
    }
  }

  // Function to update the tree visualization
  function update(source) {
    // Create a new tree layout
    const treeData = treeLayout(root);

    // Get all nodes and links
    const nodes = treeData.descendants();
    const links = treeData.links();

    // Normalize for fixed-depth
    nodes.forEach(d => d.y = d.depth * 180);

    // Initialize counter for node IDs
    let i = 0;

    // Update the nodes
    const node = svg.selectAll('g.state-viz-node')
      .data(nodes, d => d.id || (d.id = ++i));

    // Enter new nodes
    const nodeEnter = node.enter().append('g')
      .attr('class', d => `state-viz-node state-viz-node-${d.data.type}`)
      .attr('transform', d => `translate(${source.y0},${source.x0})`)
      .on('click', function (event, d) {
        // Toggle children on click
        if (d.children) {
          d._children = d.children;
          d.children = null;
        } else {
          d.children = d._children;
          d._children = null;
        }
        update(d);
      })
      .on("mouseover", function (event, d) {
        tooltip.transition()
          .duration(200)
          .style("opacity", .9);
        tooltip.html(d.data.name)
          .style("left", (event.pageX + 10) + "px")
          .style("top", (event.pageY - 28) + "px");
      })
      .on("mouseout", function () {
        tooltip.transition()
          .duration(500)
          .style("opacity", 0);
      });

    // Add circles to nodes
    nodeEnter.append('circle')
      .attr('r', 1e-6)
      .style('fill', d => d._children ? "#555" : "#fff");

    // Add text labels
    nodeEnter.append('text')
      .attr('dy', '.35em')
      .attr('x', d => d.children || d._children ? -13 : 13)
      .attr('text-anchor', d => d.children || d._children ? 'end' : 'start')
      .text(d => {
        const name = d.data.name;
        // Limit text length for better visualization
        return name.length > 50 ? name.substring(0, 50) + '...' : name;
      });

    // Update existing nodes
    const nodeUpdate = nodeEnter.merge(node);

    // Transition nodes to their new position
    nodeUpdate.transition()
      .duration(750)
      .attr('transform', d => `translate(${d.y},${d.x})`);

    // Update node attributes
    nodeUpdate.select('circle')
      .attr('r', 6)
      .style('fill', d => d._children ? "#555" : "#fff")
      .attr('cursor', 'pointer');

    // Update color of the text depending on if node has children
    nodeUpdate.select('text')
      .style('fill-opacity', 1);

    // Remove any exiting nodes
    const nodeExit = node.exit().transition()
      .duration(750)
      .attr('transform', d => `translate(${source.y},${source.x})`)
      .remove();

    // On exit, reduce the node circles size to 0
    nodeExit.select('circle')
      .attr('r', 1e-6);

    // On exit, reduce the opacity of text labels
    nodeExit.select('text')
      .style('fill-opacity', 1e-6);

    // Update the links
    const link = svg.selectAll('path.state-viz-link')
      .data(links, d => d.target.id);

    // Enter new links at parent's previous position
    const linkEnter = link.enter().insert('path', 'g')
      .attr('class', 'state-viz-link')
      .attr('d', d => {
        const o = { x: source.x0, y: source.y0 };
        return diagonal(o, o);
      });

    // Update links
    linkEnter.merge(link).transition()
      .duration(750)
      .attr('d', d => diagonal(d.source, d.target));

    // Remove any exiting links
    link.exit().transition()
      .duration(750)
      .attr('d', d => {
        const o = { x: source.x, y: source.y };
        return diagonal(o, o);
      })
      .remove();

    // Store the old positions for transition
    nodes.forEach(d => {
      d.x0 = d.x;
      d.y0 = d.y;
    });

    // Create curved links
    function diagonal(s, d) {
      return `M ${s.y} ${s.x}
                C ${(s.y + d.y) / 2} ${s.x},
                  ${(s.y + d.y) / 2} ${d.x},
                  ${d.y} ${d.x}`;
    }
  }

  // The main function that gets exported
  function updateState(raw_data) {
    // Check if SVG exists, if not initialize
    if (!svg) {
      initialize();
    }

    // Convert the JSON data to the hierarchical structure
    const hierarchyData = convertToHierarchy(raw_data);

    // Create the root hierarchy
    root = d3.hierarchy(hierarchyData, d => d.children);

    // Initialize node positions for smooth transitions
    root.x0 = height / 2;
    root.y0 = 0;

    // Collapse all nodes initially except the root
    if (root.children) {
      root.children.forEach(collapse);
    }

    // Update the visualization
    update(root);
  }

  // Return the public function
  return updateState;
}