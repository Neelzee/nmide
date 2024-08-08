import { Html } from "src/bindings/Html";
import { expect, test } from "vitest";
import { render, screen } from '@testing-library/react'
import RenderHtml from "../components/Html";

const simple_html: Html = {
  kind: "Div",
  kids: [
    {
      kind: { "Text": "Hello, World!" },
      kids: []
    }
  ]
};

const nested_div: Html = {
  kind: "Div",
  kids: [
    {
      kind: "Div",
      kids: [{
        kind: { "Text": "Hello, World!" },
        kids: []
      }
      ]
    }
  ]
};

function bfs(c: ChildNode): string[] {
  const queue: ChildNode[] = [];
  const visited_nodes: ChildNode[] = [];
  queue.unshift(c); // Adds element to start of array
  var current_node: ChildNode;
  while (queue.length > 0) {
    const l = queue.pop();
    if (l === undefined) {
      continue;
    }
    current_node = l;
    if (visited_nodes.includes(current_node)) {
      continue;
    }
    visited_nodes.push(current_node);
    current_node.childNodes.forEach(child => {
      queue.unshift(child);
    });
  }
  return visited_nodes.map(c => c.nodeName.toLowerCase()).filter(s => s !== "#text");
}

test.each([
  { input: simple_html, expected_text: "Hello, World!", expected_dom_order: ["div"] },
  { input: nested_div, expected_text: "Hello, World!", expected_dom_order: ["div", "div"] }
])("rendering: $input -> $expected_text", ({ input, expected_text, expected_dom_order }) => {
  render(<div data-testid="test-element"><RenderHtml html={input} /></div>);

  const test_dom = screen.getByTestId("test-element");
  const txt = test_dom.textContent;
  var dom_order: string[] = [];
  test_dom.childNodes.forEach(c => {
    dom_order = dom_order.concat(bfs(c));
  });
  expect(txt).toBe(expected_text);
  expect(dom_order).toStrictEqual(expected_dom_order);
});
