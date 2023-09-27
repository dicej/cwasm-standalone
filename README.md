# cwasm-standalone

This is a simple demo of how to embed a Wasmtime `cwasm` file (e.g. a
WebAssembly component compiled to native code) in a Rust program and execute it.

## Prerequisite(s)

- [Rust](https://rustup.rs/)
  - Be sure to add the `wasm32-wasi` target using `rustup target add wasm32-wasi`
  
## Building and running

`cargo run --release`

You should see "Hello, world!" printed.
