"use client"

import { useState } from "react";
import RenderHtml from "./components/Html";
import { TSHtml } from "./bindings/TSHtml";

export default function Page() {
  const [htmls, _] = useState<TSHtml[]>([]);

  return (
    <>
      {htmls.map((html) => (
        <RenderHtml
          kind={html.kind}
          kids={html.kids}
          text={html.text}
          attrs={html.attrs}
        />
      ))}
    </>
  );
}

