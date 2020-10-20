## Yew Tutorial 01
***
### Setup
```sh
cargo install wasm-pack
nmp install --global rollup
cargo new --lib yew-tutorial
```
#
### Build and Run
```sh
wasm-pack build --target web
rollup ./index.js --format iife --file ./pkg/bundle.js
python -m http.server 8080
```
#
