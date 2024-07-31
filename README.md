# Nmide

A *simple* IDE.

## Development

### Git Commit Convention

Follow [this](https://www.conventionalcommits.org/en/v1.0.0/#summary)


### Plugin Development

A plugin (`Nmlugin`), in Nmide is either a C Library, or a WASM file. Either way,
it follows the same architecture, the [Elm Architecture](https://guide.elm-lang.org/architecture/).
Using `init`, `update`, and `update` functions, the plugins will mutate the
state (`model`) of the IDE.


## Installation

### Manual

**Prerequisites**:
- Git
- Rust
- Just
- Node
- NPM


After installing all the prerequisites just run:

```shell
just build
```
