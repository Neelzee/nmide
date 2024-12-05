# Nmide

A zero-core IDE.

## Development

![latest release](https://git.app.uib.no/Nils.Fitjar/nmide/-/badges/release.svg)

### CI

> [!WARNING]
> Due to inadequate GitLab-Runner, pipelines are not in-use.

Pipeline-main: ![main pipeline status](https://git.app.uib.no/Nils.Fitjar/nmide/badges/dev/pipeline.svg)
Pipeline-dev: ![dev pipeline status](https://git.app.uib.no/Nils.Fitjar/nmide/badges/dev/pipeline.svg)

### Git Commit Convention

Follow [this](https://www.conventionalcommits.org/en/v1.0.0/#summary)


### Plugin Development

A plugin (`Nmlugin`), in Nmide is either a library or a JavaScript file. Either
way, it follows the same architecture, inspired by the
[Elm Architecture](https://guide.elm-lang.org/architecture/). Using `init`,
`update`, and `update` functions, the plugins will mutate the state (`model`)
of the IDE.

Plugins are considered to be pure, so should not have their own internal state.


#### Examples

#### JavaScript

Minimal JavaScript Plugin
```JavaScript
window.plugins.set(
  "PluginName",
  {
    // Sets the initial state
    init: () => {
      return [];
    },
    // Renders based on the state
    view: (model) => {
      return { kind: "Frag", kids: [], attrs: [], text: null },
    },
    // Returns changes to be made to the state.
    update: (msg, model) => {
      return [];
    }
  }
);
```

For a more thorough example, see `nmide-core/plugins`.


#### Rust

Minimal Rust Plugin

```rust
use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rvec, sabi_extern_fn,
    std_types::{ROption, RString, RVec},
};
use nmide_std_lib::{
    attr::rattr::RAttr,
    html::rhtml::RHtml,
    map::rmap::RMap,
    msg::rmsg::RMsg,
    NmideStandardLibrary_Ref, NmideStdLib,
};

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

For a move in depth example see the `nmide-plugin` folder.


## Installation

> [!WARNING]
> None of these binaries are signed

The recommended way of installing this app, is the [Manual](#manual) way.

### Windows

- [MSI](TBA)
- [NSIS](TBA)


### Linux

- [AppImage](TBA)
- [Deb](TBA)
- [RPM](TBA)


### macOS

- [DMG](TBA)

### Manual

#### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/start/prerequisites/)
- [NodeJS](https://nodejs.org/en)

> [!NOTE]
> Instead of installing NodeJS directly, I recommend installing it using
> [nvm](https://github.com/nvm-sh/nvm)


After installing all the prerequisites just run:

```shell
cd nmide-core/ && npm i && npm run tauri build
```

And find the binary in: `nmide-core/src-tauri/target/release/bundle`
