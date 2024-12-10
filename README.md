# Nmide

A zero-core IDE.

### CI

> [!WARNING]
> Due to recent migration from GitLab to GitHub, workflow/pipelines are not up to date.


### Plugin Development

A plugin (`Nmlugin`), in Nmide is either a library or a JavaScript file. Either
way, it follows the same architecture, inspired by the
[Elm Architecture](https://guide.elm-lang.org/architecture/). Using `init`,
`update`, and `update` functions, the plugins will mutate the state (`model`) of
the IDE.

Plugins are considered to be pure, so should not have their own internal state.


### Plugin Examples

#### JavaScript

Minimal JavaScript Plugin
```JavaScript
window.plugins.set(
  "PluginName",
  {
    // Sets the initial state
    init: () => [],
    // Renders based on the state
    view: (model) => {
      return { kind: "Frag", kids: [], attrs: [], text: null },
    },
    // Returns changes to be made to the state.
    update: (msg, model) => []
  }
);
```

For a more thorough example, see [core/plugins](https://github.com/Neelzee/nmide/tree/main/plugins).


#### Rust

Minimal Rust Plugin
```rust
#[export_root_module]
pub fn get_library() -> NmideStandardLibrary_Ref {
    NmideStdLib { init, view, update }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn init() -> RMap {
    RMap::new()
}

#[sabi_extern_fn]
pub fn view(model: RMap) -> RHtml {
  RHtml::Frag(Vec::new(), Vec::new())
}

#[sabi_extern_fn]
pub fn update(msg: RMsg, model: RMap) -> RMap {
    RMap::new()
}
```

The Rust example contains an extra function, and some annotations, but this is
to avoid undefined behavior that can be caused by Rust ABI.

For a more thorough example, see [core/plugins](https://github.com/Neelzee/nmide/tree/main/plugins).


## Installation

The recommended way of installing this app, is the [Manual](#manual) way.

### Manual

#### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/start/prerequisites/)
- [NodeJS](https://nodejs.org/en)

After installing all the prerequisites run these two commands:

```shell
cd libs/js-utils && npm i
```
```shell
cd core/ && npm i && npm run tauri build -- -- features ide
```
