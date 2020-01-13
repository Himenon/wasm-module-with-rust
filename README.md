
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
wasm-bindgen target/wasm32-unknown-unknown/debug/wasm_module_with_rust.wasm --out-dir .
```