[![Build Status](https://travis-ci.com/yever/rusty-typescript.svg?branch=master)](https://travis-ci.com/yever/rusty-typescript)

# rusty-typescript
A re-implementation of TypeScript in Rust.

This is a very experimental project in its initial phase.
Its goal is to create a [WebAssembly](https://webassembly.org/)
implementation of [TypeScript](https://www.typescriptlang.org/)
written in [Rust](https://www.rust-lang.org/).

The approach taken here is to inline the WebAssembly binary into the
JavaScript, and bundle everything into one TypeScript file (with type
checking and linting disabled).
This makes the result platform-independent (browser vs. `nodejs`) and easy
to inject into the [source code of TypeScript](https://github.com/Microsoft/TypeScript).
This way, the new implementation can be verified against the same tests used by TypeScript itself.

## How to run it?

You would need to have `nodejs` and `npm` installed as well as the Rust toolchain
for the target `wasm32-unknown-unknown`.

To build the WebAssembly artifact, run:
```sh
$ npm run build
```

To inject the artifact into the TypeScript source code (included here as a git submodule), run:
```sh
$ npm run inject
```

To run the TypeScript tests against the injected source code, run:
```sh
$ npm test
```
