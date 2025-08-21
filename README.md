# Nmide

A zero-core IDE.

This is repository contains my master project and thesis, under the working
title `Creating a Modular IDE`. This project was started, because a research
group at the University of Bergen is developing an experimental research
programming language. Being an experimental language, and having all of it's
tooling, like the compiler is still under development, poses a unique challenge
for developing an IDE for it, so flexibility is key. To achieve this, this IDE
is _zero-core_. The only functionality it has, is to load and manage modules.

Everything, from the file explorer, to editor, are features one or more modules
has to create.

Rust was chosen, because it is low-level, so the idea was it would allow for an
easier way to create bindings to other languages like C, to eventually allow for
a language agnostic module architecture. This has yet to be implemented.

Currently, the IDE only supports Modules made in JavaScript or Rust. It does
achieves this by using the [Tauri](https://tauri.app/) framework. Wherein one of
the features is being able to implement the frontend using JavaScript. This
makes it very easy to implement a system for JavaScript Modules.


## Subversion

This repository is also maintained in a subversion repository, ~~hence the
`.svn` folder~~, _pushing_ to subversion does not occur as often as to GitHub,
so the repository can be accessed [here](https://github.com/Neelzee/nmide).


# Installation

You can either build the IDE from source, or by using the released binaries.
In either case, there are two different instances of the IDE.

**Empty core**: Without any of the modules
**IDE**: With all of modules

The different modules can be found in [modules](https://github.com/Neelzee/nmide/tree/main/modules),
and are added by changing the `Modules.toml` file, and running `make modules`.

## Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/start/prerequisites/)
- [NodeJS](https://nodejs.org/en)
- [Bun](https://bun.sh/)
- [Make (Optional)](https://www.gnu.org/software/make/)

`make` is optional, but it is recommended for building the IDE.


## Building empty core from source

After installing all the prerequisites:

1. Install the JavaScript libraries: in `libs/javascript/*` by using make:
  ```shell
make install-deps
  ```

  1. If you don't have `make`, you have to go each library in `libs/javascript`,
  and run `bun i` and `bun link`. See [this](https://bun.sh/docs/cli/link) for
  more information.

2. Install the node dependencies in `core`:
  ```shell
  bun i
  ```

If you want to run a development build, simply run the following in `core`:
```shell
bun run tauri dev
```

If you want to build it, instead run this, in `core`:
```shell
bun run tauri build
```

This will build the application specific to your OS, and the resulting binary
can be found in `core/src-tauri/target/release/bundle`


## Building IDE from source using make

```shell
make ide
```

You'll find the executable in a folder pertaining to your OS in:
`core/src-tauri/target/release/bundle/`


## Building IDE from source

1. Install the JavaScript libraries: in `libs/javascript/*`, by going into each
  folder and running `bun i` and `bun link`. See
  [this](https://bun.sh/docs/cli/link) for more information.

2. Install the node dependencies in `core`:
  ```shell
  bun i
  ```

3. You have to add each Rust module as a dependency in
  `core/src-tauri/Cargo.toml`, and import it, and add it in
  `core/src-tauri/target/module_reg.rs`. For the JavaScript modules, you have
  to build them using `bun run build`, and add the `build/index.js` file to
  `build/modules.js`.

If you want to run a development build, simply run the following in `core`:
```shell
bun run tauri dev
```

If you want to build it, instead run this, in `core`:
```shell
bun run tauri build
```

You'll find the executable in a folder pertaining to your OS in:
`core/src-tauri/target/release/bundle/`


# Module Development

A module is either compile time or runtime module. Either way, it
follows the same architecture. A module exposes an `init` and `handler` method.

Modules are considered to be pure, so should not have their own internal state,
any guarantees the IDE gives, hinges on this.


## Module Examples


### JavaScript

Minimal JavaScript Module
```JavaScript
import { emptyCm, installModule } from "@nmide/js-utils";
installModule({
  init: async (_) => {},
  handler: async (_, __) => {},
});
```

For a more thorough example, see [core/modules](https://github.com/Neelzee/nmide/tree/main/modules).


### Rust

Minimal Compile time Rust Module
```rust
use async_trait::async_trait;
use core_std_lib::{attrs::Attr, core::Core, event::Event};

pub struct ModuleBuilder;

impl core_module_lib::ModuleBuilder for ModuleBuilder {
    fn build(self) -> impl core_module_lib::Module {
        Module
    }
}

pub struct Module;

#[async_trait]
impl core_module_lib::Module for Module {
    fn name(&self) -> &str {
        "trivial module"
    }

    async fn init(&self, _: Box<dyn Core>) {}

    async fn handler(&self, _: Event, _: Box<dyn Core>) {}
}
```

The compile-time-module in Rust, needs to provide a method of creating the
module instance, hence the `ModuleBuilder`, this needs to be a `pub`-lic struct,
and needs to implement the `ModuleBuilder` trait. This is to make _installation_
of compile-time-modules easier, as we can simply create a script which
actually adds the module during build-time.

Minimal Runtime Rust Module
```rust
use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use async_ffi::{FfiFuture, FutureExt};
use core_module_lib::rs_module::{ModuleRef, RCore_CTO, RustModule};
use foreign_std_lib::{
    core::rs_core_modification::RCoreModification, event::rs_event::REvent, state::rs_state::RValue,
};

#[export_root_module]
pub fn get_library() -> ModuleRef {
    RustModule { init, handler }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn init(_: RCore_CTO) -> FfiFuture<()> {
  async move {}.into_ffi()
}

#[sabi_extern_fn]
pub fn handler(_: REvent, _: RCore_CTO) -> FfiFuture<()> {
    async move {}.into_ffi()
}
```

The Rust runtime example contains an extra function, and some annotations, but this is
to avoid undefined behavior that can be caused by Rust ABI.

For a more thorough example, see [core/modules](https://github.com/Neelzee/nmide/tree/main/plugins).

