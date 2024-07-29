# Nmide

## Development



## Setup

### Windows

TODO: Add exe


### Linux

TODO: Find out how to do this, _easily_


### Mac

TODO: Find out how hard this is.


## How to build

## Prerequisites

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Tauri requisites](https://beta.tauri.app/guides/prerequisites/)
- [Node](https://docs.npmjs.com/downloading-and-installing-node-js-and-npm)


### Windows/Linux

```shell
just build
```

The executable will be in `src-tauri/target/release`


TODO: Move this to its own wiki?

## React _Frontend_

- TODO: Add info about frontend


## Rust Backend

### Structure

#### osops

Os-operations. Managing folder and files.


#### workspace

Abstraction of file-management, communication between the frontend and backend happen at this level.

No function call from the frontend should be directly to an os-operation.
