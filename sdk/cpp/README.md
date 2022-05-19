This directory contains generated bindings for the cpp sdk. I'll probably remove it from git and have it generated on the fly in `Makefile.toml` but wanted to document how they were generated for now as I'm working on it.

First, you need to install `cxxbridge` to generate the bindings:

```
cargo install cxxbridge-cmd
```

Then run the following commands from the root of this repo to generate the files.

```
cxxbridge src/cpp.rs > sdk/cpp/rollbar.cc
cxxbridge src/cpp.rs --header > sdk/cpp/rollbar.h
```
