# mostly stolen from https://github.com/bevyengine/bevy/tree/main/examples#wasm
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_pacman --out-dir wasm/target --target web target/wasm32-unknown-unknown/release/pacman.wasm
basic-http-server wasm