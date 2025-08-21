/*
 * Entry point for the Nmide frontend.
 *
 * Handles JavaScript modules, frontend runtimes, and event passing to the
 * backend.
 *
 * # JavaScript modules
 *
 * A JavaScript module is just a script tag, with a JavaScript source. Normally,
 * modules are installed during compile time. Installation, in this context,
 * simply means that the modules are added to the
 * `window.__nmideConfig__.modules` map, which is handled by the `installModule`
 * function, maintained by @nmide/js-utils (in lib/Module.ts). Compile time
 * installation, then, means that the module scripts are bundled into one large
 * script, loaded _before_ the main script, `../index.ts`, which allows modules
 * to do some pre-core stuff, if that is wanted. But since the module scripts
 * themself install themself by adding a module into the module map.
 *
 *
 * # Frontend runtimes
 *
 * The frontend can have several different _runtimes_. In this context, we use
 * the term `runtime`, to mean a system for handling invocation of `init` and
 * `handler` methods on a module. This system was developed, to help future
 * foreign modules to be integrated into the frontend without having to create a
 * wrapper modules. An example, is Gleam modules. In Gleam, all the different
 * types are classes, and as such, do not map directly to their corresponding
 * JavaScript types. So, instead of having to translate this for every module,
 * the idea of a `runtime` was created, to allow for a Gleam `runtime` to be
 * created, which would have done the translation at the point of entry and
 * exit.
 *
 *
 * # Event passing
 *
 * Regardless of where an event is thrown from, be it the frontend or backend,
 * it ends up in the frontend first. It is then passed to backend, _after_ the
 * frontend runtime systems has received the event.
 *
 *   Event from frontend
 *     ┌───────────┐
 *     │           │
 *     │           │
 *     │           │
 *     ▼           │
 *  ┌──────────────┐      Event from backend       ┌─────────────┐
 *  │              │◄──────────────────────────────│             │
 *  │              │                               │             │
 *  │   Frontend   │                               │   Backend   │
 *  │              │   Event passed to backend     │             │
 *  │              │──────────────────────────────►│             │
 *  └──────────────┘                               └─────────────┘
 *
 * @see [Tauri Architecture](https://tauri.app/concept/architecture/)
 * @see JavaScript modules `libs/javascript/utils/lib/Modules.ts`
 * @see [Foreign modules](https://github.com/Neelzee/nmide/wiki/nomenclature)
 */

import { run } from "./lib/run";
import app from "./lib/app";
import config from "./lib/config";

document.addEventListener("DOMContentLoaded", () => {
  run(app, config);
});
