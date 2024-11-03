# Nmide

A *simple* IDE.

## Development

![latest release](https://git.app.uib.no/Nils.Fitjar/nmide/-/badges/release.svg)

### CI

Pipeline-main: ![main pipeline status](https://git.app.uib.no/Nils.Fitjar/nmide/badges/dev/pipeline.svg)
Pipeline-dev: ![dev pipeline status](https://git.app.uib.no/Nils.Fitjar/nmide/badges/dev/pipeline.svg)

### Git Commit Convention

Follow [this](https://www.conventionalcommits.org/en/v1.0.0/#summary)


### Plugin Development

A plugin (`Nmlugin`), in Nmide is either a C Library, ~~or a JS file.~~ Either way,
it follows the same architecture, the [Elm Architecture](https://guide.elm-lang.org/architecture/).
Using `init`, `update`, and `update` functions, the plugins will mutate the
state (`model`) of the IDE.


## Installation

### Manual

**Prerequisites**:
- Git
- Rust
- ~~Just~~
- NPM


After installing all the prerequisites just run:

```shell
cd nmide-core/ && npm i && npm run tauri build
```

And find the executable at: `nmide-core/src-tauri/target`
