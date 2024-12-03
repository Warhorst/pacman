# mostly stolen from https://github.com/bevyengine/bevy/tree/main/examples#wasm
cd pacman/
cargo build --release --target wasm32-unknown-unknown
cd ..
wasm-bindgen --out-name wasm_pacman --out-dir web/pacman/target --target web pacman/target/wasm32-unknown-unknown/release/pacman.wasm
cp -r pacman/assets/ web/pacman/