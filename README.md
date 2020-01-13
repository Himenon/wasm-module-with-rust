
## Build

```
cargo build --target=wasm32-unknown-unknown
```

### Error tips

**error: cannot find attribute `wasm_bindgen` in this scope**

```diff
[package]
+ edition = "2018"

[lib]
```

### wasm-bindgen

```bash
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_module_with_rust.wasm --out-dir lib
```

### [expand](https://github.com/dtolnay/cargo-expand)

逆コンパイル

```bash
cargo expand --target=wasm32-unknown-unknown > expanded.rs
```