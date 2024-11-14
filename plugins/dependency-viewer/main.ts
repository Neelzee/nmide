import { HtmlBuilder, THtml, TMap, TMsg } from "@nmide/js-utils"

// @ts-ignore
window.plugins.set(
  "DependencyViewer",
  {
    init: (): TMap => {
      return [];
    },
    view: (model: TMap): THtml => {
      return new HtmlBuilder()
        .kids([
          new HtmlBuilder()
            .kind("Div")
            .attrs([{ Id: "container" }]),
          new HtmlBuilder()
            .kind("Script")
            .attrs([{ Src: "https://cdn.jsdelivr.net/npm/d3@7" }]),
          new HtmlBuilder()
            .kind("Script")
            .text(
              `// Declare the chart dimensions and margins.
            const width = 640;
            const height = 400;
            const marginTop = 20;
            const marginRight = 20;
            const marginBottom = 30;
            const marginLeft = 40;

            // Declare the x (horizontal position) scale.
            const x = d3.scaleUtc()
                .domain([new Date("2023-01-01"), new Date("2024-01-01")])
                .range([marginLeft, width - marginRight]);

            // Declare the y (vertical position) scale.
            const y = d3.scaleLinear()
                .domain([0, 100])
                .range([height - marginBottom, marginTop]);

            // Create the SVG container.
            const svg = d3.create("svg")
                .attr("width", width)
                .attr("height", height);

            // Add the x-axis.
            svg.append("g")
                .attr("transform", \`translate(0,\${height - marginBottom})\`)
                .call(d3.axisBottom(x));

            // Add the y-axis.
            svg.append("g")
                .attr("transform", \`translate(\${ marginLeft }, 0)\`)
                .call(d3.axisLeft(y));

            // Append the SVG element.
            container.append(svg.node());`
            )
        ])
        .build();
    },
    update: (msg: TMsg, model: TMap): TMap => {
      return [];
    },
  }
);
