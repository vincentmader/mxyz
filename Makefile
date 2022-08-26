# test:
# 	make client
# 	./scripts/build
all:
	make engine
	make client
	make server
engine:
	./bin/rebuild_db
	cd src && \
		cargo run -p mxyz-engine # --release
server:
	./bin/rebuild_db
	cd src && \
		cargo run -p mxyz-server # --release
client:
	cd ./src/mxyz-client && \
		cargo build --target wasm32-unknown-unknown # --release
	cd src && \
		wasm-bindgen --target web --out-dir "./mxyz-server/static/pkg" "./target/wasm32-unknown-unknown/debug/mxyz_client.wasm"

stats:
	./bin/stats/fuse.py
	./bin/stats/show_nr_of_lines_in_proj.py
dc:
	cd src && \
		docker-compose up # --build -d
