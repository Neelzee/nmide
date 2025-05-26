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


## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/start/prerequisites/)
- [NodeJS](https://nodejs.org/en)
- [Bun](https://bun.sh/)
- [Make](https://www.gnu.org/software/make/)

After installing all the prerequisites:

1. Install the js-library: in `libs/js-utils`
  ```shell
  bun i
  ```

2. Install the node dependencies in `core`:
  ```shell
  bun i
  ```

3. Add the wanted modules in the `core/Modules.toml` file

4. Install the modules
  ```shell
  make modules
  ```

5. Build the project:
```shell
bun run tauri build
```

You'll find the executable in a folder pertaining to your OS in:
`core/src-tauri/target/release/bundle/`

## Development

If you want to develop, debug or test different modules, you can do step `1` to
`4`, and just run the development build by using the following command:

```shell
bun run tauri dev
```

This will run a development instance of the application.

## Module Development

A module is either compile time or runtime module. Either way, it
follows the same architecture. A module exposes an `init` and `handler` method.

Modules are considered to be pure, so should not have their own internal state,
any guarantees the IDE gives, hinges on this.


### Module Examples

#### JavaScript

Minimal JavaScript Module
```JavaScript
import { emptyCm, installModule } from "@nmide/js-utils";
installModule({
  init: async (_) => {},
  handler: async (_, __) => {},
});
```

For a more thorough example, see [core/modules](https://github.com/Neelzee/nmide/tree/main/modules).


#### Rust

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

