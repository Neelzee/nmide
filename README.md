# Nmide

A zero core IDE.

This is repository contains my master project and thesis, under the working
title `Creating a Modular IDE`. This project was started, because a research
group at the University of Bergen is developing an experimental research
programming language. Being an experimental language, and having all of it's
tooling, like the compiler is still under development, poses a unique challenge
for developing an IDE for it, so flexibility is key. To achieve this, this IDE
is _zero core_. The only functionality it has, is to load and manage plugins.

Everything, from the file explorer, to editor, are features one or more plugins
has to create.

Rust was chosen, because it is low-level, so the idea was it would allow for an
easier way to create bindings to other languages like C, to eventually allow for
a language agnostic plugin architecture. This has yet to be implemented.

Currently, the IDE only supports Plugins made in JavaScript or Rust. It does
achieves this by using the [Tauri](https://tauri.app/) framework. Wherein one of
the features is being able to implement the frontend using JavaScript. This
makes it very easy to implement a system for JavaScript Plugins.


## Installation

### Prerequisites

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

## CI

> [!WARNING]
> Due to recent migration from GitLab to GitHub, workflow/pipelines are not up to date.


## Plugin Development

A plugin is either a library or a JavaScript file. Either way, it
follows the same architecture, inspired by the
[Elm Architecture](https://guide.elm-lang.org/architecture/). Using `init`,
`update`, and `update` functions, the plugins will mutate the state (`model`) of
the IDE.

Plugins are considered to be pure, so should not have their own internal state,
any guarantees the IDE gives, hinges on this.


### Plugin Examples

#### JavaScript

Minimal JavaScript Plugin
```JavaScript
window.plugins.set(
  "PluginName",
  {
    init: () => [],
    view: _ => {
      return { kind: "Frag", kids: [], attrs: [], text: null },
    },
    update: (_, __) => []
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

