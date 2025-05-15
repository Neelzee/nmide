import {
  click,
  cls,
  Core,
  CoreModification,
  emptyCm,
  Event,
  HtmlBuilder,
  id,
  isPrimAnd,
  isTBool,
  isTList,
  isTObj,
  isTStr,
  mkPrimEvent,
  primDec,
  StateBuilder,
  tList,
  tObjLookupOr,
  UiBuilder,
  ValueList
} from "@nmide/js-utils";
import { initializeGraph } from "./dag";

const controls_div = new HtmlBuilder()
  .attrs(
    cls("dv controls"),
    id("dv-controls")
  )
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
    new HtmlBuilder()
      .kids(
        new HtmlBuilder()
          .kind("label")
          .text("Hide Graph")
          .attrs(click(mkPrimEvent("")))
      )
  );

const legend_div = new HtmlBuilder()
  .attrs(id("packageLegend"), cls("dv legend"));

const package_filter_div = new HtmlBuilder()
  .attrs(id("packageFilter"), cls("dv package-filter"))
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
    return new UiBuilder()
      .add(
        controls_div,
      )
      .add(
        legend_div,
      )
      .add(
        package_filter_div,
      )
      .add(
        new HtmlBuilder()
          .attrs(id("visualization"), cls("dv")),
      )
      .build();
  },
  handler: async (evt: Event, core: Core) => {
    if (!isPrimAnd(evt, "graph")) return emptyCm();

    const val = (await core.state())["dv-init"];
    const toggled = isTBool(val) ? val.bool : true;

    const mods = (toggled
      ? new UiBuilder()
        .add_attr(cls("show-dv"), "dv-controls")
        .add_attr(cls("show-dv"), "packageLegend")
        .add_attr(cls("show-dv"), "packageFilter")
        .add_attr(cls("show-dv"), "visualization")
      : new UiBuilder()
        .rem_attr(cls("show-dv"), "dv-controls")
        .rem_attr(cls("show-dv"), "packageLegend")
        .rem_attr(cls("show-dv"), "packageFilter")
        .rem_attr(cls("show-dv"), "visualization")).build(new StateBuilder().set("dv-init", !toggled));

    const { args } = primDec(evt);

    if (args === null) return mods;

    if (!isTList(args)) return mods;

    const data = args.list
      .filter(v => isTObj(v))
      .map((obj) => {
        const id = obj["name"]?.["str"];
        return {
          id,
          name: id,
          source: id,
          targets: tObjLookupOr<ValueList>("dependencies")
            (tList([]))
            (obj).list
            .filter(v => isTStr(v))
            .map(s => s.str),
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
      .filter(
        ({ source, target }) =>
          nodes.find(n => n.id === source) !== undefined
          && nodes.find(n => n.id === target) !== undefined
      );

    let graphContext;
    try {
      graphContext = initializeGraph(nodes, links)
    } catch (e) {
      window.__nmideConfig__.log.error(`Error when initializing graph: ${JSON.stringify(e)}`);
    }

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

    window.addEventListener('resize', () => {
      try {
        handleResize();
      } catch (e) {
        window.__nmideConfig__
          .log
          .error(`Error when handleResize: ${JSON.stringify(e)}`);
      }
    });

    return mods;
  }
};

export default Module;