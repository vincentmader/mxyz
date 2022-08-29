# test:
# 	make client
# 	./scripts/build
build:
	./bin/rebuild_db
	make build_client
	# make build_engine
	make build_server
build_client:
	cd ./src/mxyz-client && \
		cargo build --target wasm32-unknown-unknown # --release
	cd ./src && \
		wasm-bindgen --target web --out-dir "./mxyz-server/static/pkg" "./target/wasm32-unknown-unknown/debug/mxyz_client.wasm"
build_engine:
	./bin/rebuild_db
	cd ./src && \
		cargo build -p mxyz-engine # --release
build_server:
	./bin/rebuild_db
	cd ./src && \
		cargo build -p mxyz-server # --release

run:
	make build_client
	# make build_engine
	make run_server
run_engine:
	./bin/rebuild_db
	cd ./src && \
		cargo run -p mxyz-engine # --release
run_server:
	./bin/rebuild_db
	cd ./src && \
		cargo run -p mxyz-server # --release

stats:
	./bin/stats/fuse.py
	./bin/stats/show_nr_of_lines_in_proj.py
dc:
	cd ./src/ && \
		docker-compose up # --build -d
