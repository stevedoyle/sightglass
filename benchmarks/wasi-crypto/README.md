# WASI-Crypto

This benchmark uses [wasi-crypto](https://github.com/WebAssembly/wasi-crypto), a
proposed [WebAssembly System Interface](https://github.com/WebAssembly/WASI) API
for cryptography.

Because wasi-crypto supports many different cryptography algorithms which vary
widely in performance, there are multiple benchmarks and each benchmark is
targeted at a specific algorithm.

## Steps to run this benchmark from the top sightglass directory

Run all wasi-crypto benchmarks:

```
cargo run -- benchmark --engine engines/wasmtime/libengine.so --engine-flags=--wasi-modules=experimental-wasi-crypto -- benchmarks/wasi-crypto/wasi-crypto-*.wasm
```

Run a single wasi-crypto benchmark:

```
cargo run -- benchmark --engine engines/wasmtime/libengine.so --engine-flags=--wasi-modules=experimental-wasi-crypto -- benchmarks/wasi-crypto/wasi-crypto-aesgcm.wasm
```

## Notes

- You must build the wasmtime engine with the wasi-crypto feature enabled. This can be done using: `cargo build --release -p wasmtime-bench-api --features wasi-crypto` when building the engine.
- You must use `--engine-flags=--wasi-modules=experimental-wasi-nn` when running this benchmark. Otherwise Wasmtime won't link the required wasi-crypto functions.
