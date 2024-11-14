import { NmluginVerified as Nmlugin } from "@nmide/js-utils";

const TrivialPlugin: Nmlugin = {
  init: () => [],
  view: (_) => { return { kind: "Frag", kids: [], attrs: [], text: null, }; },
  update: (_, __) => []
};

export default TrivialPlugin;
