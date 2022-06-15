engine:
	cargo run -p mxyz-engine # --release
server:
	cargo run -p mxyz-server # --release
client:
	cd ./mxyz-client && cargo build --target wasm32-unknown-unknown # --release
	wasm-bindgen --target web --out-dir "./mxyz-server/static/pkg" "./target/wasm32-unknown-unknown/debug/mxyz_client.wasm"

