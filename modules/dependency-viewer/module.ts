import {
  cls,
  Core,
  CoreModification,
  emptyCm,
  Event,
  HtmlBuilder,
  id,
  isPostInit,
  isPrimAnd,
  isTList,
  primDec,
  UiBuilder
} from "@nmide/js-utils";
import { initializeGraph } from "./dag";

const controls_div = new HtmlBuilder()
  .attrs(cls("controls"))
  .kids(
    new HtmlBuilder()
      .kind("button")
      .attrs(id("zoomIn"))
      .text("+"),
    new HtmlBuilder()
      .kind("button")
      .attrs(id("zoomOut"))
      .text("-"),
    new HtmlBuilder()
      .kind("button")
      .attrs(id("resetZoom"))
      .text("Reset View"),
    new HtmlBuilder()
      .kind("button")
      .attrs(id("resetZoom"))
      .text("Find Cycles"),
    new HtmlBuilder()
      .kids(
        new HtmlBuilder()
          .kind("label")
          .text("Force Strength:"),
        new HtmlBuilder()
          .kind("input")
          .attrs(
            { type: "range" },
            id("chargeSlider"),
            { custom: ["min", "-200"] },
            { custom: ["max", "-10"] },
            { custom: ["value", "-30"] },
            { custom: ["step", "-5"] },
          ),
        new HtmlBuilder()
          .kind("span")
          .attrs(id("chargeValue"))
      ),
    new HtmlBuilder()
      .kids(
        new HtmlBuilder()
          .kind("label")
          .text("Link Distance:"),
        new HtmlBuilder()
          .kind("input")
          .attrs(
            { type: "range" },
            id("linkDistanceSlider"),
            { custom: ["min", "50"] },
            { custom: ["max", "1200"] },
            { custom: ["value", "300"] },
            { custom: ["step", "10"] },
          ),
        new HtmlBuilder()
          .kind("span")
          .attrs(id("linkDistanceValue"))
      ),
    new HtmlBuilder()
      .kids(
        new HtmlBuilder()
          .kind("label")
          .text("Node Size Scale:"),
        new HtmlBuilder()
          .kind("input")
          .attrs(
            { type: "range" },
            id("nodeSizeSlider"),
            { custom: ["min", "1"] },
            { custom: ["max", "5"] },
            { custom: ["value", "2"] },
            { custom: ["step", "0.5"] },
          ),
        new HtmlBuilder()
          .kind("span")
          .attrs(id("nodeSizeValue"))
      ),
  );

const legend_div = new HtmlBuilder().attrs(id("packageLegend"));

const package_filter_div = new HtmlBuilder()
  .attrs(id("packageFilter"))
  .kids(
    new HtmlBuilder().kind("h3").text("Package Filter"),
    new HtmlBuilder().attrs(id("packageCheckboxes")),
    new HtmlBuilder().kids(
      new HtmlBuilder().kind("button").attrs(id("selectAll")).text("Select All"),
      new HtmlBuilder().kind("button").attrs(id("deselectAll")).text("Deselect All"),
    ),
  );

const Module = {
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
  handler: async (evt: Event, __: Core) => {
    console.log("Dp: ", evt);
    if (isPostInit(evt)) {
      console.log("post-init");
      return new UiBuilder()
        .add(
          controls_div,
          "content"
        )
        .add(
          legend_div,
          "content"
        )
        .add(
          package_filter_div,
          "content"
        )
        .add(
          new HtmlBuilder()
            .attrs(id("visualization")),
          "content"
        )
        .build();
    }

    if (!isPrimAnd(evt, "graph")) return emptyCm();

    const { args } = primDec(evt);

    if (args === null) return emptyCm();

    if (!isTList(args)) return emptyCm();

    const data = args.list
      .filter(v => v !== "null")
      .filter(v => "obj" in v)
      .map(({ obj }) => {
        const id = obj["name"]?.["str"];
        return {
          id,
          name: id,
          source: id,
          targets: obj["dependencies"]?.["list"]?.map(o => o?.["str"]),
        };
      });

    const nodes = data.map(({ id, name }) => {
      return { id, name };
    });

    const links = data.map(({ source, targets }) => {
      return targets.map(target => {
        return {
          source,
          target,
        };
      });
    })
      .flat()
      .filter(({ source, target }) => nodes.find(n => n.id === source) !== undefined && nodes.find(n => n.id === target) !== undefined);


    let graphContext = initializeGraph(nodes, links);

    function handleResize() {
      if (graphContext === undefined) {
        return emptyCm();
      }

      const container = document.getElementById("content");
      if (container === null) {
        console.log("Error on graphing: could not find element with id: `content`");
        return emptyCm();
      }
      const width = container.clientWidth;
      const height = container.clientHeight;

      d3.select("#visualization svg")
        .attr("width", width)
        .attr("height", height)
        .attr("viewBox", [0, 0, width, height]);

      if (graphContext.simulation) {
        graphContext.simulation
          .force("center", d3.forceCenter(width / 2, height / 2))
          .force("x", d3.forceX(width / 2).strength(0.05))
          .force("y", d3.forceY(height / 2).strength(0.05))
          .alpha(0.3)
          .restart();
      }

      if (graphContext.svg && graphContext.zoom) {
        graphContext.svg.call(
          graphContext.zoom.transform,
          d3.zoomIdentity.translate(width / 2, height / 2)
        );
      }
    }

    window.addEventListener('resize', handleResize);

    /*
    const resizeObserver = new ResizeObserver(entries => {
      handleResize();
    });

    resizeObserver.observe(document.getElementById("content")!!);
    */

    return emptyCm();
  }
};

export default Module;