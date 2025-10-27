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
  tBool,
  tList,
  tObjLookupOr,
  UiBuilder,
  ValueList
} from "@nmide/js-utils";
import { initializeGraph } from "./dag";
import * as d3 from "d3";

const controls_div = new HtmlBuilder()
  .attrs(
    cls("dv controls hide-dv"),
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
          .kind("button")
          .text("Hide Graph")
          .attrs(click(mkPrimEvent("graph-toggle"))),
      )
  );

const legend_div = new HtmlBuilder()
  .attrs(id("packageLegend"), cls("dv legend hide-dv"));

const package_filter_div = new HtmlBuilder()
  .attrs(
    id("packageFilter"),
    cls("dv package-filter hide-dv")
  )
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
  init: async (core: Core): Promise<void> => {
    await core.registerHandler("DependencyViewer", "graph")
      .catch(err => console.error("error from module: ", err));
    await core.registerHandler("DependencyViewer", "graph-toggle")
      .catch(err => console.error("error from module: ", err));
    await core.sendModification(new UiBuilder()
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
          .attrs(id("visualization"), cls("dv hide-dv")),
      )
      .build());
  },
  handler: async (evt: Event, core: Core): Promise<void> => {
    const val = (await core.state())["dv-init"];
    const toggled = isTBool(val) ? val.bool : true;

    let state_builder = new StateBuilder().add("dv-init", tBool(!toggled));
    const init_val = (await core.state())["dv-init-init"];
    const inited = isTBool(init_val) ? init_val.bool : false;
    let builder = (!toggled
      ? new UiBuilder()
        .add_attr(cls("hide-dv"), "dv-controls")
        .add_attr(cls("hide-dv"), "packageLegend")
        .add_attr(cls("hide-dv"), "packageFilter")
        .add_attr(cls("hide-dv"), "visualization")
        .add_attr(cls("hide-dv"), "canvas")
      : new UiBuilder()
        .rem_attr(cls("hide-dv"), "dv-controls")
        .rem_attr(cls("hide-dv"), "packageLegend")
        .rem_attr(cls("hide-dv"), "packageFilter")
        .rem_attr(cls("hide-dv"), "canvas")
        .rem_attr(cls("hide-dv"), "visualization"));
    if (isPrimAnd(evt, "graph-toggle")) {
      if (!inited) {
        state_builder = state_builder.add("dv-init-init", tBool(true));
      }
      await core.sendModification(builder.build(state_builder));
      return;
    }
    if (!isPrimAnd(evt, "graph")) return;

    const mods = builder.build(state_builder);

    if (inited) {
      await core.sendModification(mods);
      return;
    }

    const { args } = primDec(evt);

    if (args === null) {
      await core.sendModification(mods);
      return;
    }

    if (!isTList(args)) {
      await core.sendModification(mods);
      return;
    }

    const data = args.list
      .filter(v => isTObj(v))
      .map(obj => {
        const id = obj.obj["name"]?.["str"];
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

    let container = document.getElementById("content");
    if (container === null) {
      console.log("Error on graphing: could not find element with id: `content`");
    }
    const w = container?.clientHeight;
    const width = w === undefined ? window.innerWidth : w;
    const h = container?.clientHeight;
    const height = h === undefined ? window.innerHeight : h;

    let graphContext;
    try {
      graphContext = initializeGraph(nodes, links, width, height)
    } catch (e) {
      console.log("Error: ", e);
      window.__nmideConfig__.log.error(`Error when initializing graph: ${JSON.stringify(e)}`);
    }

    function handleResize() {
      if (graphContext === undefined) {
        return;
      }

      d3.select("#visualization svg");

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

    await core.sendModification(mods);
  }
};

export default Module;
