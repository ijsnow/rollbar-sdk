> This repository is a WIP proof of concept.

# Rollbar SDK

This repository contains a rust library that contains logic to send items to Rollbar APIs. It currently has the following examples:

- [A nextjs application](./examples/nextjs) that can consume this library built to web assembly.
- [A nodejs program](./examples/nextjs) that can consume this library built to a native compilation target.
- [A c++ program](./examples/cpp) that can consume this library as a dynamic library.

## Architecture

`rollbar-sdk` is a single library (not a rust workspace) that contains code that knows how to communicate with (for now) web assembly and nodejs runtimes.

This library uses `wasm-pack` to build a directory that can be used as a `node_module`. The standard `wasm-bindgen` library is used for the glue between the web assembly runtime and the rust runtime. Both of the mentioned tools are supported by the Rust Webassembly Working Group which is supported by a number of companies including Mozilla.

For node, the library uses [`neon-bindings`](https://neon-bindings.com/) to facilitate communication between the rust runtime and the node runtime. It's supported by a number of companies including 1Password for their desktop (Electron) application.

## See it in action

### Prerequisites

- [Rust must be installed.](https://rustup.rs/)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) for building to web assembly.
- A modern version of `nodejs`
- `python2` must be executable in your `$PATH`. It is a dependency of `node-gyp` which is a tool used by node for managaging native addon modules.
- To handle dependency management for c/c++, this project uses [cmake](https://cmake.org/). Ensure you have at least version 3.19 installed.
- To make things easier, this project uses `cargo-make` to run tasks. Install with:

```
cargo install --force cargo-make
```

Next, make a file named `.env` in the root of this repository and add the following.

```
POST_TOKEN=<your project's post_client_item here>
```

## Running Wasm

[`./examples/nextjs`](./examples/nextjs) contains a nextjs application where this library is consumed as a web assembly module. To see it in action run the following from the `rollbar-sdk` directory: 

```shell
cargo make run-nextjs-example
```

Next, open [http://localhost:3000](http://localhost:3000), fiddle with the form fields, and send it 🏂! Once you submit the form, check out the item in the rollbar application.

## Running Node

[`./examples/nodejs`](./examples/nextjs) contains a simple node program where this library is consumed as a native module. Feel free to fiddle with the program at [`./examples/nodejs/index.js`.](./examples/nodejs/index.js) To see it in action, run the following:

```shell
cargo make run-nodejs-example
```

## Running c++

[`./examples/cpp`](./examples/cpp) contains a simple c++ program where this library is consumed as a dynamic library. Feel free to fiddle with the program at [`./examples/cpp/main.cpp`.](./examples/cpp/main.cpp) To see it in action, run the following:

```shell
cargo make run-cpp-example
```

## Caveats

So far, I've found some caveats, particularly with the web assembly target:

- Node has support for loading web assembly files, but it is currently experimental. This means that users would have to ensure the experiment flag be added to their `node` invocation. However, this is not a requirement when build natively with the `neon` library.
- The `wasm-pack` build tool and it's underlying support program (`wasm-bindgen-cli`) don't support compiling to multiple targets. This means that some custom/hacky scripts would have to be added if a separate package for browser and nodejs isn't desired (i.e. just a single `rollbar-sdk` package that the current `rollbar` package can consume.
  - https://github.com/rustwasm/wasm-pack/issues/427
  - https://github.com/rustwasm/wasm-pack/issues/313
- Web assembly must be consumed from a module ready application. The consumer's web application must be using a bundler configured to consume web assembly or be served from a server that supports module loading. This won't be a problem for most modern web applications. This also means that the application can't be bundled the number of ways that it currently is [here.](https://github.com/rollbar/rollbar.js/tree/master/release)
- While popular companies such as 1Password use `neon`, it's ecosystem is still a little slow. Rollbar would likely need to contribute here, which is not inherently a negative!
  - In order to use the standard `serde` library to make passing around objects easier, I had to use an outdated version of `neon` [(waiting on this PR to be merged)](https://github.com/matrix-org/neon-serde/pull/2).
- For c/c++, consumers of this library will need rust installed when building their application.
