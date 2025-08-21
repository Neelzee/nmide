import * as fs from "fs";
import * as d3 from "d3";
import { JSDOM } from "jsdom";
import { getEventName, objAdd, tList, tStr, Value, ValueList, ValueObj } from "@nmide/js-utils";
import svg2img from "svg2img";

import type { NamedDependency } from "./rsm-grapher/rsm-invoker/bindings/NamedDependency";
import type { Event } from "./rsm-grapher/rsm-invoker/bindings/Event";
import { pipe } from "fp-ts/lib/function";

function getEventType(event: Event): string {
  if (typeof event === "string") {
    return "core";
  } else if ("event" in event) {
    return "event";
  } else if ("dialogEvent" in event) {
    return "dialog";
  } else if ("dialogFile" in event) {
    return "file";
  } else if ("coreResponse" in event) {
    return "response";
  }
  return "unknown";
}

interface Link {
  source: string;
  target: string;
  event: Event;
  eventType: string;
  eventName: string;
  linkIndex?: number;
  total?: number;
}

interface Node {
  id: string;
  group: number;
}

interface Graph {
  nodes: Node[];
  links: Link[];
}

const rsm_data: NamedDependency[] = JSON.parse(fs.readFileSync("./build/result.json", "utf-8"));
const jsm_data: NamedDependency[] = JSON.parse(fs.readFileSync("./build/jsm_result.json", "utf-8"));
const copy_data: (NamedDependency & { kind: "Rust" | "JavaScript" })[] = JSON.parse(JSON.stringify([...rsm_data.map(ndep => { return { ...ndep, kind: "Rust" } }), ...jsm_data.map(ndep => { return { ...ndep, kind: "JavaScript" } })]));
const data = [...rsm_data.map(ndep => { return { ...ndep, kind: "Rust" } }), ...jsm_data.map(ndep => { return { ...ndep, kind: "JavaScript" } })];

const s_t = data.flatMap(n_d => {
  const consumer = n_d.name;
  return n_d.consuming.map(({ event_name }) => {
    return {
      consumer,
      consuming: {
        event_name,
      }
    };
  });
});

const providers = data.flatMap(n_d => {
  const provider = n_d.name;
  return n_d.providing.map(evt => {
    return {
      provider,
      providing: evt
    };
  });
});

const links: Link[] = [];

const consumedEvents = new Set<string>();
const providedEvents = new Set<string>();

s_t.forEach(({ consuming }) => {
  consumedEvents.add(consuming.event_name);
});

providers.forEach(({ providing }) => {
  providedEvents.add(getEventName(providing));
});

s_t.forEach(({ consumer, consuming }) => {
  const source = consumer;
  let foundProvider = false;

  providers.forEach(({ provider, providing }) => {
    const target = provider;
    if (consuming.event_name === getEventName(providing)) {
      links.push({
        source,
        target,
        event: providing,
        eventType: getEventType(providing),
        eventName: getEventName(providing)
      });
      foundProvider = true;
    }
  });

  if (!foundProvider && consuming.event_name) {
    if (!providedEvents.has(consuming.event_name)) {
      const unknownEvent: Event = {
        "event": {
          event: consuming.event_name,
          args: null
        }
      };

      links.push({
        source: "Unknown Source",
        target: consumer,
        event: unknownEvent,
        eventType: "unknown",
        eventName: consuming.event_name
      });
    }
  }
});

const linksByPair = new Map<string, Link[]>();
links.forEach(link => {
  const key = `${link.source}-${link.target}`;
  if (!linksByPair.has(key)) {
    linksByPair.set(key, []);
  }
  linksByPair.get(key)!.push(link);
});

linksByPair.forEach(pairLinks => {
  const total = pairLinks.length;
  pairLinks.forEach((link, index) => {
    link.linkIndex = index;
    link.total = total;
  });
});

const nodeSet = new Set<string>();
links.forEach(link => {
  nodeSet.add(link.source);
  nodeSet.add(link.target);
});

const nodes: Node[] = Array.from(nodeSet).map(id => {
  if (id === "Unknown Source") {
    return { id, group: 3 };
  }

  const isProvider = providers.some(p => p.provider === id);
  const isConsumer = s_t.some(c => c.consumer === id);

  let group = 2;
  if (isProvider && !isConsumer) {
    group = 1;
  } else if (isProvider && isConsumer) {
    group = 0;
  }

  return { id, group };
});

const graph: Graph = {
  nodes,
  links
};

function generateSVG(): string {
  const dom = new JSDOM('<!DOCTYPE html><html><body></body></html>');
  const document = dom.window.document;
  global.document = document;

  const width = 3840;
  const height = 2160;
  const margin = { top: 100, right: 500, bottom: 100, left: 100 };

  const svg = d3.select(document.body)
    .append('svg')
    .attr('xmlns', 'http://www.w3.org/2000/svg')
    .attr('width', width)
    .attr('height', height)
    .attr('viewBox', `0 0 ${width} ${height}`)
    .attr('style', 'max-width: 100%; height: auto;');

  const g = svg.append('g')
    .attr('transform', `translate(${margin.left}, ${margin.top})`);

  const simulation = d3.forceSimulation(graph.nodes)
    .force('link', d3.forceLink(graph.links).id((d: any) => d.id).distance(700))
    .force('charge', d3.forceManyBody().strength(-1800))
    .force('center', d3.forceCenter((width - margin.right) / 2, height / 2))
    .stop();

  for (let i = 0; i < 600; ++i) simulation.tick();

  const eventTypeColor = d3.scaleOrdinal()
    .domain(['event', 'dialog', 'file', 'core', 'response', 'unknown'])
    .range(['#1f77b4', '#ff7f0e', '#2ca02c', '#d62728', '#9467bd', '#8c564b']);

  // Updated linkArc function with more pronounced curves for better label separation
  const linkArc = (d: any) => {
    const dx = d.target.x - d.source.x;
    const dy = d.target.y - d.source.y;
    const dr = Math.sqrt(dx * dx + dy * dy);

    // Handle case where source and target might be the same
    if (dr === 0) {
      // Create a larger loop for self-references
      return `M${d.source.x},${d.source.y} C${d.source.x + 80},${d.source.y - 80} ${d.source.x + 80},${d.source.y + 80} ${d.source.x},${d.source.y}`;
    }

    if (d.total === 1) {
      // More pronounced curve for single links
      const cpX = (d.source.x + d.target.x) / 2;
      const cpY = (d.source.y + d.target.y) / 2 - dr * 0.25; // Increased curve

      return `M${d.source.x},${d.source.y} Q${cpX},${cpY} ${d.target.x},${d.target.y}`;
    }

    // For multiple links between the same points, increase separation
    const offset = d.linkIndex - (d.total - 1) / 2;

    const midX = (d.source.x + d.target.x) / 2;
    const midY = (d.source.y + d.target.y) / 2;

    const normX = -dy / dr;
    const normY = dx / dr;

    // Increased offset multiplier for better separation
    const cpX = midX + normX * offset * 100;
    const cpY = midY + normY * offset * 100;

    return `M${d.source.x},${d.source.y} Q${cpX},${cpY} ${d.target.x},${d.target.y}`;
  };

  {
    const link = g.append('g')
      .attr('class', 'links')
      .selectAll('path')
      .data(graph.links)
      .enter().append('path')
      .attr('d', linkArc)
      .attr('fill', 'none')
      .attr('stroke', (d: any) => eventTypeColor(d.eventType))
      .attr('stroke-opacity', 0.8)
      .attr('stroke-width', 2);

    svg.append('defs').selectAll('marker')
      .data(['event', 'dialog', 'file', 'core', 'response', 'unknown'])
      .enter().append('marker')
      .attr('id', d => `arrow-${d}`)
      .attr('viewBox', '0 -5 10 10')
      .attr('refX', 45)
      .attr('refY', 0)
      .attr('markerWidth', 12)
      .attr('markerHeight', 12)
      .attr('orient', 'auto')
      .append('path')
      .attr('d', 'M0,-5L10,0L0,5')
      .attr('fill', d => eventTypeColor(d));

    link.attr('marker-end', (d: any) => `url(#arrow-${d.eventType})`);
  }

  // Create a collision detection system to avoid label overlaps
  const labelPositions: Array<{ x: number, y: number, width: number, height: number }> = [];

  // Function to check if a new label would overlap with existing ones
  const wouldOverlap = (x: number, y: number, width: number, height: number): boolean => {
    const buffer = 10; // Extra buffer space around labels

    for (const pos of labelPositions) {
      if (
        x - buffer < pos.x + pos.width + buffer &&
        x + width + buffer > pos.x - buffer &&
        y - buffer < pos.y + pos.height + buffer &&
        y + height + buffer > pos.y - buffer
      ) {
        return true;
      }
    }
    return false;
  };

  // Improved getLabelPosition function with collision avoidance
  const getLabelPosition = (d: any) => {
    // Get the base path points
    const dx = d.target.x - d.source.x;
    const dy = d.target.y - d.source.y;
    const dr = Math.sqrt(dx * dx + dy * dy);

    // Get estimated label dimensions
    const labelWidth = Math.max(d.eventName.length * 12, 60);
    const labelHeight = 30;

    // Handle the case where source and target might be the same node
    if (dr === 0) {
      return findNonOverlappingPosition(d.source.x + 50, d.source.y + 50, labelWidth, labelHeight);
    }

    let cpX, cpY;
    if (d.total > 1) {
      const offset = d.linkIndex - (d.total - 1) / 2;

      const midX = (d.source.x + d.target.x) / 2;
      const midY = (d.source.y + d.target.y) / 2;

      const normX = -dy / dr;
      const normY = dx / dr;

      // Increase offset separation for better spacing
      cpX = midX + normX * offset * 100;
      cpY = midY + normY * offset * 100;
    } else {
      // For single links, use a simple curve with more pronounced arc
      cpX = (d.source.x + d.target.x) / 2;
      cpY = (d.source.y + d.target.y) / 2 - dr * 0.25;
    }

    // Try different points along the curve for better label positioning
    let bestPosition: { x: number, y: number } | null = null;
    let bestT = 0.5; // Default t-value (middle of curve)

    // Try different positions along the path (0.3 to 0.7 of the path)
    for (let t = 0.3; t <= 0.7; t += 0.1) {
      // Bezier curve formula for quadratic curves
      let x = (1 - t) * (1 - t) * d.source.x + 2 * (1 - t) * t * cpX + t * t * d.target.x;
      let y = (1 - t) * (1 - t) * d.source.y + 2 * (1 - t) * t * cpY + t * t * d.target.y;

      // Skip NaN positions
      if (isNaN(x) || isNaN(y)) continue;

      // Check if this position would be good (no overlap)
      if (!wouldOverlap(x - labelWidth / 2, y - labelHeight / 2, labelWidth, labelHeight)) {
        bestPosition = { x, y };
        bestT = t;
        break;
      }
    }

    // If we found a good position, use it
    if (bestPosition) {
      // Register this position to avoid future overlaps
      labelPositions.push({
        x: bestPosition.x - labelWidth / 2,
        y: bestPosition.y - labelHeight / 2,
        width: labelWidth,
        height: labelHeight
      });
      return bestPosition;
    }

    // Default to middle of the curve if all else fails
    const t = 0.5;
    let x = (1 - t) * (1 - t) * d.source.x + 2 * (1 - t) * t * cpX + t * t * d.target.x;
    let y = (1 - t) * (1 - t) * d.source.y + 2 * (1 - t) * t * cpY + t * t * d.target.y;

    // Handle NaN with fallback
    if (isNaN(x) || isNaN(y)) {
      // Try to find a non-overlapping position near the midpoint
      return findNonOverlappingPosition(
        (d.source.x + d.target.x) / 2,
        (d.source.y + d.target.y) / 2,
        labelWidth,
        labelHeight
      );
    }

    // Final position - we'll take this even if it overlaps
    const finalPos = findNonOverlappingPosition(x, y, labelWidth, labelHeight);
    return finalPos;
  };

  // Helper function to find a non-overlapping position near a target point
  const findNonOverlappingPosition = (x: number, y: number, width: number, height: number) => {
    // Try the original position first
    if (!wouldOverlap(x - width / 2, y - height / 2, width, height)) {
      labelPositions.push({
        x: x - width / 2,
        y: y - height / 2,
        width,
        height
      });
      return { x, y };
    }

    // Try positions at increasing distances in a spiral pattern
    const spiralPoints = 16; // Number of points to try in the spiral
    const maxRadius = 200;   // Maximum radius to try

    for (let i = 1; i <= spiralPoints; i++) {
      const radius = (i / spiralPoints) * maxRadius;
      const angle = i * (Math.PI * 0.6); // Create a spiral by increasing angle

      const newX = x + Math.cos(angle) * radius;
      const newY = y + Math.sin(angle) * radius;

      if (!wouldOverlap(newX - width / 2, newY - height / 2, width, height)) {
        labelPositions.push({
          x: newX - width / 2,
          y: newY - height / 2,
          width,
          height
        });
        return { x: newX, y: newY };
      }
    }

    // If all else fails, just use the original position and register it
    labelPositions.push({
      x: x - width / 2,
      y: y - height / 2,
      width,
      height
    });
    return { x, y };
  };

  // Modified rendering of label backgrounds with improved z-ordering
  // Draw link paths first (lowest z-index)
  const link = g.append('g')
    .attr('class', 'links')
    .selectAll('path')
    .data(graph.links)
    .enter().append('path')
    .attr('d', linkArc)
    .attr('fill', 'none')
    .attr('stroke', (d: any) => eventTypeColor(d.eventType))
    .attr('stroke-opacity', 0.8)
    .attr('stroke-width', 2);

  link.attr('marker-end', (d: any) => `url(#arrow-${d.eventType})`);

  // Create label group with all labels (higher z-index than paths)
  const labelGroup = g.append('g').attr('class', 'labels');

  // Sort links by source-target distance for better label positioning
  // This helps prioritize labels for shorter connections
  const sortedLinks = [...graph.links].sort((a, b) => {
    const distA = Math.hypot(
      (a.source as any).x - (a.target as any).x,
      (a.source as any).y - (a.target as any).y
    );
    const distB = Math.hypot(
      (b.source as any).x - (b.target as any).x,
      (b.source as any).y - (b.target as any).y
    );
    return distB - distA; // Process shorter links last (they'll be on top)
  });

  // Draw label backgrounds and text for all links
  sortedLinks.forEach((d: any) => {
    const pos = getLabelPosition(d);
    const width = Math.max(d.eventName.length * 12, 60);
    const height = 30;

    // Draw background rectangle
    labelGroup.append('rect')
      .attr('x', pos.x - width / 2)
      .attr('y', pos.y - 15)
      .attr('width', width)
      .attr('height', height)
      .attr('fill', 'white')
      .attr('fill-opacity', 0.9)
      .attr('rx', 8)
      .attr('ry', 8)
      .attr('stroke', eventTypeColor(d.eventType))
      .attr('stroke-width', 1.5);

    // Draw text on top of the background
    labelGroup.append('text')
      .attr('x', pos.x)
      .attr('y', pos.y + 5)
      .attr('text-anchor', 'middle')
      .attr('font-family', 'Arial')
      .attr('font-size', '16px')
      .attr('font-weight', 'bold')
      .attr('fill', eventTypeColor(d.eventType))
      .text(d.eventName)
      .attr('pointer-events', 'none');
  });

  const node = g.append('g')
    .attr('class', 'nodes')
    .selectAll('g')
    .data(graph.nodes)
    .enter().append('g');

  const nodeColor = d3.scaleOrdinal()
    .domain(['0', '1', '2', '3'])
    .range(['#9467bd', '#2ca02c', '#1f77b4', '#e74c3c']);

  // Make sure nodes are drawn after labels so they're on top
  node.append('circle')
    .attr('r', (d: any) => d.id === "Unknown Source" ? 40 : 30)
    .attr('cx', (d: any) => d.x)
    .attr('cy', (d: any) => d.y)
    .attr('fill', (d: any) => nodeColor(d.group.toString()))
    .attr('stroke', '#fff')
    .attr('stroke-width', 4);

  // Draw text on top of everything
  node.append('text')
    .attr('dx', (d: any) => d.x + 35)
    .attr('dy', (d: any) => d.y + 10)
    .text((d: any) => d.id)
    .attr('font-family', 'Arial')
    .attr('font-size', '24px')
    .attr('font-weight', (d: any) => d.id === "Unknown Source" ? 'bold' : 'normal')
    .attr('fill', '#333')
    .attr('stroke', 'white')
    .attr('stroke-width', 5)
    .attr('stroke-linejoin', 'round')
    .attr('paint-order', 'stroke')
    .attr('pointer-events', 'none');

  const legend = svg.append('g')
    .attr('class', 'node-legend')
    .attr('transform', `translate(${width - margin.right + 50}, ${margin.top})`);

  legend.append('text')
    .attr('x', 0)
    .attr('y', -30)
    .text('Node Types')
    .attr('font-family', 'Arial')
    .attr('font-size', '28px')
    .attr('font-weight', 'bold');

  const nodeTypes = [
    { type: "Provider & Consumer", group: 0 },
    { type: "Provider", group: 1 },
    { type: "Consumer", group: 2 },
    { type: "Unknown Source", group: 3 }
  ];

  nodeTypes.forEach((item, i) => {
    legend.append('circle')
      .attr('r', 12)
      .attr('cx', 0)
      .attr('cy', i * 50)
      .attr('fill', nodeColor(item.group.toString()));

    legend.append('text')
      .attr('x', 25)
      .attr('y', i * 50 + 6)
      .text(item.type)
      .attr('font-family', 'Arial')
      .attr('font-size', '24px')
      .attr('fill', '#333');
  });

  const eventLegend = svg.append('g')
    .attr('class', 'event-legend')
    .attr('transform', `translate(${width - margin.right + 50}, ${margin.top + 250})`);

  eventLegend.append('text')
    .attr('x', 0)
    .attr('y', -30)
    .text('Event Types')
    .attr('font-family', 'Arial')
    .attr('font-size', '28px')
    .attr('font-weight', 'bold');

  const eventTypes = [
    "event", "dialog", "file", "core", "response", "unknown"
  ];

  eventTypes.forEach((type, i) => {
    eventLegend.append('line')
      .attr('x1', 0)
      .attr('y1', i * 50)
      .attr('x2', 60)
      .attr('y2', i * 50)
      .attr('stroke', eventTypeColor(type))
      .attr('stroke-width', 5);

    eventLegend.append('text')
      .attr('x', 75)
      .attr('y', i * 50 + 6)
      .text(type.charAt(0).toUpperCase() + type.slice(1))
      .attr('font-family', 'Arial')
      .attr('font-size', '24px')
      .attr('fill', '#333');
  });

  return document.body.innerHTML;
}

const svgString = generateSVG();

fs.writeFileSync('./build/module-dependencies.svg', svgString);

console.log('SVG exported to ./build/module-dependencies.svg');

svg2img(svgString, (err, buff) => {
  const pngPath = "./build/module-dependencies.png";
  console.log(`PNG exported to ${pngPath}`);
  if (err !== undefined && err !== null) {
    console.error("Svg conversion error: ", err);
  }
  fs.writeFileSync(pngPath, buff);
})

console.log(`Graph contains ${nodes.length} nodes and ${links.length} links.`);
console.log(`Event types in the graph: ${[...new Set(links.map(l => l.eventType))].join(', ')}`);

const nodeCounts = {
  both: nodes.filter(n => n.group === 0).length,
  providers: nodes.filter(n => n.group === 1).length,
  consumers: nodes.filter(n => n.group === 2).length,
  unknown: nodes.filter(n => n.group === 3).length
};
console.log('Node counts by role:', nodeCounts);

const eventTypeCounts = links.reduce((acc: Record<string, number>, link) => {
  acc[link.eventType] = (acc[link.eventType] || 0) + 1;
  return acc;
}, {});
console.log('Link counts by event type:', eventTypeCounts);

fs.writeFileSync('./build/module-dependencies.json', JSON.stringify(graph, null, 2));
console.log('Graph data exported to ./build/module-dependencies.json');

console.log("Translating graph data to DependencyViewer format");

const dp_format: { event: string, args: ValueList } = {
  event: "graph",
  args: {
    list: copy_data.map(ndep => {
      const obj: ValueObj = { obj: {} };
      const dependencies = ndep.consuming
        .map(c => {
          return copy_data.find(({ providing }) => providing.map(getEventName).includes(c.event_name));
        })
        .filter(x => x !== undefined)
        .map(s => `${s.kind}.${s.name}`);
      return pipe(
        obj,
        o => objAdd(o, "name", tStr(`${ndep.kind}.${ndep.name}`)),
        o => objAdd(o, "dependencies", tList(dependencies)),
      );
    }),
  }
};

fs.writeFileSync('./build/dependency-viewer-format.json', JSON.stringify(dp_format, null, 2));
console.log('Graph data exported to ./build/dependency-viewer-format.json');
