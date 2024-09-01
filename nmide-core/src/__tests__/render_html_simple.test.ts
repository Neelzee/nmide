import { expect, test } from 'vitest';
import RenderHtml from '../components/Html';
import { render, screen } from '@testing-library/react'

test("rendering with text should display text", () => {
  render(
    RenderHtml(
      {
        kind: "Div",
        attrs: [],
        kids: [],
        text: "Hello, World!"
      }),
  );

  const txt = screen.getByText("Hello, World!").textContent;

  expect(txt).toBe(
    "Hello, World!"
  );
});
