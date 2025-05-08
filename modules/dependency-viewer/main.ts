import {
  Core,
  CoreModification,
  emptyCm,
  Event,
  HtmlBuilder,
  id,
  installModule,
  UiBuilder
} from "@nmide/js-utils";
installModule(
  {
    name: "DependencyViewer",
    init: async (core: Core): Promise<CoreModification> => {
      const script = document.createElement("script");
      script.type = "text/javascript";
      script.src = "https://cdn.jsdelivr.net/npm/d3@7"
      document.head.appendChild(script);
      await core.registerHandler("DependencyViewer", "graph")
        .catch(err => console.error("error from module: ", err));
      await core.registerHandler("DependencyViewer", "post-init")
        .catch(err => console.error("error from module: ", err));
      return emptyCm();
    },
    handler: async ({ event, args }: Event, __: Core) => {
      if (event === "post-init") {
        return new UiBuilder()
          .add(
            new HtmlBuilder()
              .attrs(id("visualization")),
            "content"
          )
          .build();
      }
      if (event !== "graph" || args === null || args === "null" || !("list" in args)) {
        return emptyCm();
      }

      const data = args.list
        .filter(v => v !== "null")
        .filter(v => "obj" in v)
        .map(({ obj }) => {
          const id = obj["name"]?.["str"];
          return {
            id,
            name: id,
            source: id,
            targets: obj["dependencies"]?.["list"]?.map(o => o?.["str"])
          };
        });

      const nodes = data.map(({ id, name }) => {
        return { id, name };
      });

      const links: { source: string, target: string }[] = data.flatMap(({ source, targets }) => {
        return targets.map(target => {
          return {
            source,
            target
          };
        }).filter(l => l.targets !== undefined);
      });

      // Create SVG container
      const svg = d3.select("#visualization")
        .append("svg")
        .attr("id", "canvas");

      // Create force simulation
      const simulation = d3.forceSimulation(nodes)
        .force("link", d3.forceLink(links).id(d => d.id).distance(100))
        .force("charge", d3.forceManyBody().strength(-200))

      // Create links
      const link = svg.append("g")
        .selectAll("path")
        .data(links)
        .enter()
        .append("path")
        .attr("class", "link");

      // Create nodes
      const node = svg.append("g")
        .selectAll(".node")
        .data(nodes)
        .enter()
        .append("g")
        .attr("class", "node");

      // Add circles to nodes
      node.append("circle")
        .attr("r", 6);

      // Add text labels
      node.append("text")
        .attr("dx", 12)
        .attr("dy", ".35em")
        .text(d => d.name);

      // Add drag capability
      node.call(d3.drag()
        .on("start", dragstarted)
        .on("drag", dragged)
        .on("end", dragended));

      // Update positions on each tick of the simulation
      simulation.on("tick", () => {
        link.attr("d", d => {
          return `M${d.source.x},${d.source.y}A${30},${30} 0 0,1 ${d.target.x},${d.target.y}`;
        });

        node.attr("transform", d => `translate(${d.x},${d.y})`);
      });

      // Drag functions
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

      return emptyCm();
    }
  }
);
