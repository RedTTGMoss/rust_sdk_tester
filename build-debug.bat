rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1
copy target\wasm32-wasip1\debug\*.wasm .