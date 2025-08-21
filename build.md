# README

The `source` folder contains the source files for this project.

The `ide-bundle` and `empty-bundle` folder contains `appimage`, `deb`, and `rpm`
folders, with the IDE and _empty_ IDE files.

If you're on an operative system which cannot use `appimage`, `deb` or `rpm`,
then you have to build it manually.

This project is open sourced, and available on [GitHub](https://github.com/Neelzee/nmide).


### Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Tauri](https://tauri.app/start/prerequisites/)
- [NodeJS](https://nodejs.org/en)
- [Bun](https://bun.sh/)
- [Make (Optional)](https://www.gnu.org/software/make/)

`make` is optional, but it is recommended for building the IDE.


### Windows:

If you are on windows, try getting make by following [this guide](https://stackoverflow.com/questions/32127524/how-to-install-and-use-make-in-windows)


## Manual build

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


### Building IDE from source using make

```shell
make ide
```

You'll find the executable in a folder pertaining to your OS in:
`core/src-tauri/target/release/bundle/`


### Building IDE from source

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
