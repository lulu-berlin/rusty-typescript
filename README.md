[![Build Status](https://travis-ci.com/yever/rusty-typescript.svg?branch=master)](https://travis-ci.com/yever/rusty-typescript)
[![Gitter](https://badges.gitter.im/rusty-typescript/community.svg)](https://gitter.im/rusty-typescript/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge)

# rusty-typescript
A TypeScript compiler written in Rust.

This is a very experimental project in its initial phase.
Its goal is to create a [WebAssembly](https://webassembly.org/)
implementation of [TypeScript](https://www.typescriptlang.org/)
written in [Rust](https://www.rust-lang.org/).

Note that this is still a normal TypeScript compiler, compiling TypeScript code into JavaScript. It only leverages Rust and WebAssembly for better performance.

The approach taken here is to inline the WebAssembly binary into the
JavaScript, and bundle everything into one TypeScript file (with type
checking and linting disabled).
This makes the result platform-independent (browser vs. `nodejs`) and easy
to inject into the [source code of TypeScript](https://github.com/Microsoft/TypeScript).
This way, the new implementation can be verified against the same tests used by TypeScript itself.

## How to run it?

You would need to have `nodejs` and `npm` installed. Running `npm install` will automatically install the Rust toolchain and [wasm-pack](https://github.com/rustwasm/wasm-pack) (using [wasm-pack-npm](https://github.com/robertohuertasm/wasm-pack-npm)). 

You might need to add the cargo binary directory to your `PATH`, like so:
```sh
$ export PATH="$HOME/.cargo/bin:$PATH"
```

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

## Contribute

Contributions are very welcome. Just open an issue, add a comment to an existing one or fork and make a pull request.

Rewriting the entire codebase of TypeScript is a very big (and fun!) project. There is a lot to do and enough for whoever is interested in giving a hand. Join me and let's make it happen :)

![rusty-typescript](https://media1.tenor.com/images/2ceb8a9146957d119d377e7c63b3d8fd/tenor.gif)
