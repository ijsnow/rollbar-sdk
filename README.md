> This repository is a WIP proof of concept.

# Rollbar SDK Kit

This repository contains a rust library that contains logic to send items to Rollbar APIs. It currently has the following examples:

- [A nextjs application](./examples/nextjs) that can consume this library built to web assembly.
- [A nodejs program](./examples/nextjs) that can consume this library built to a native compilation target.

## Architecture

`rollbar-sdk` is a single library (not a rust workspace) that contains code that knows how to communicate with (for now) web assembly and nodejs runtimes.

This library uses `wasm-pack` to build a directory that can be used as a `node_module`. The standard `wasm-bindgen` library is used for the glue between the web assembly runtime and the rust runtime. Both of the mentioned tools are supported by the Rust Webassembly Working Group which is supported by a number of companies including Mozilla.

For node, the library uses [`neon-bindings`](https://neon-bindings.com/) to facilitate communication between the rust runtime and the node runtime. It's supported by a number of companies including 1Password for their desktop (Electron) application.

## Caveats

So far, I've found some caveats, particularly with the web assembly target:

- Node has support for loading web assembly files, but it is currently experimental. This means that users would have to ensure the experiment flag be added to their `node` invocation. However, this is not a requirement when build natively with the `neon` library.
- The `wasm-pack` build tool and it's underlying support program (`wasm-bindgen-cli`) don't support compiling to multiple targets. This means that some custom/hacky scripts would have to be added if a separate package for browser and nodejs isn't desired (i.e. just a single `rollbar-sdk` package that the current `rollbar` package can consume.
  - https://github.com/rustwasm/wasm-pack/issues/427
  - https://github.com/rustwasm/wasm-pack/issues/313
- While popular companies such as 1Password use `neon`, it's ecosystem is still a little slow. Rollbar would likely need to contribute here, which is not inherently a negative!
  - In order to use the standard `serde` library to make passing around objects easier, I had to use an outdated version of `neon` [(waiting on this PR to be merged)](https://github.com/matrix-org/neon-serde/pull/2).
