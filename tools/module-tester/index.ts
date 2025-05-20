import * as fs from "fs";
import * as d3 from "d3";
import { JSDOM } from "jsdom";
import { getEventName } from "@nmide/js-utils";

import type { NamedDependency } from "./rsm-grapher/rsm-invoker/bindings/NamedDependency";
import type { Event } from "./rsm-grapher/rsm-invoker/bindings/Event";

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

const data: NamedDependency[] = JSON.parse(fs.readFileSync("./build/result.json", "utf-8"));

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

  const linkArc = (d: any) => {
    if (d.total === 1) {
      const dx = d.target.x - d.source.x;
      const dy = d.target.y - d.source.y;
      const dr = Math.sqrt(dx * dx + dy * dy);

      const cpX = (d.source.x + d.target.x) / 2;
      const cpY = (d.source.y + d.target.y) / 2 - dr * 0.2;

      return `M${d.source.x},${d.source.y} Q${cpX},${cpY} ${d.target.x},${d.target.y}`;
    }

    const dx = d.target.x - d.source.x;
    const dy = d.target.y - d.source.y;
    const dr = Math.sqrt(dx * dx + dy * dy);

    const offset = d.linkIndex - (d.total - 1) / 2;

    const midX = (d.source.x + d.target.x) / 2;
    const midY = (d.source.y + d.target.y) / 2;

    const normX = -dy / dr;
    const normY = dx / dr;

    const cpX = midX + normX * offset * 80;
    const cpY = midY + normY * offset * 80;

    return `M${d.source.x},${d.source.y} Q${cpX},${cpY} ${d.target.x},${d.target.y}`;
  };

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

  const getLabelPosition = (d: any) => {
    if (d.total === 1) {
      const dx = d.target.x - d.source.x;
      const dy = d.target.y - d.source.y;
      const dr = Math.sqrt(dx * dx + dy * dy);

      const cpX = (d.source.x + d.target.x) / 2;
      const cpY = (d.source.y + d.target.y) / 2 - dr * 0.2;

      const t = 0.5;
      const x = (1 - t) * (1 - t) * d.source.x + 2 * (1 - t) * t * cpX + t * t * d.target.x;
      const y = (1 - t) * (1 - t) * d.source.y + 2 * (1 - t) * t * cpY + t * t * d.target.y;

      const pos = { x, y };
      if (isNaN(pos.x) || isNaN(pos.y)) {
        return { x: d.target.x + dx, y: d.target.y + dy };
      }
      return pos;
    }

    const t = 0.5;
    const dx = d.target.x - d.source.x;
    const dy = d.target.y - d.source.y;
    const dr = Math.sqrt(dx * dx + dy * dy);

    const offset = d.linkIndex - (d.total - 1) / 2;

    const midX = (d.source.x + d.target.x) / 2;
    const midY = (d.source.y + d.target.y) / 2;

    const normX = -dy / dr;
    const normY = dx / dr;

    const cpX = midX + normX * offset * 80;
    const cpY = midY + normY * offset * 80;

    const x = (1 - t) * (1 - t) * d.source.x + 2 * (1 - t) * t * cpX + t * t * d.target.x;
    const y = (1 - t) * (1 - t) * d.source.y + 2 * (1 - t) * t * cpY + t * t * d.target.y;

    const pos = { x, y };
    if (isNaN(pos.x) || isNaN(pos.y)) {
      return { x: d.target.x + dx, y: d.target.y + dy };
    }
    return pos;
  };

  g.append('g')
    .attr('class', 'link-label-backgrounds')
    .selectAll('rect')
    .data(graph.links)
    .enter().append('rect')
    .attr('x', (d: any) => {
      const pos = getLabelPosition(d);
      return pos.x - 100;
    })
    .attr('y', (d: any) => {
      const pos = getLabelPosition(d);
      return pos.y - 20;
    })
    .attr('width', 200)
    .attr('height', 40)
    .attr('fill', 'white')
    .attr('fill-opacity', 0.9)
    .attr('rx', 8)
    .attr('ry', 8);

  g.append('g')
    .attr('class', 'link-labels')
    .selectAll('text')
    .data(graph.links)
    .enter().append('text')
    .attr('x', (d: any) => {
      const pos = getLabelPosition(d);
      return pos.x;
    })
    .attr('y', (d: any) => {
      const pos = getLabelPosition(d);
      return pos.y + 7;
    })
    .attr('text-anchor', 'middle')
    .attr('font-family', 'Arial')
    .attr('font-size', '22px')
    .attr('font-weight', 'bold')
    .attr('fill', (d: any) => eventTypeColor(d.eventType))
    .text((d: any) => d.eventName)
    .attr('pointer-events', 'none');

  const node = g.append('g')
    .attr('class', 'nodes')
    .selectAll('g')
    .data(graph.nodes)
    .enter().append('g');

  const nodeColor = d3.scaleOrdinal()
    .domain(['0', '1', '2', '3'])
    .range(['#9467bd', '#2ca02c', '#1f77b4', '#e74c3c']);

  node.append('circle')
    .attr('r', (d: any) => d.id === "Unknown Source" ? 40 : 30)
    .attr('cx', (d: any) => d.x)
    .attr('cy', (d: any) => d.y)
    .attr('fill', (d: any) => nodeColor(d.group.toString()))
    .attr('stroke', '#fff')
    .attr('stroke-width', 4);

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
