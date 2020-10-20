## Yew Tutorial 01
***
<br />

### Setup
```
cargo install wasm-pack
nmp install --global rollup
cargo new --lib yew-tutorial
```
---
<br />


### Build and Run
```
wasm-pack build --target web
rollup ./index.js --format iife --file ./pkg/bundle.js
python -m http.server 8080
```
---
<br />