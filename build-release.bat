rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
copy target\wasm32-wasip1\release\*.wasm .