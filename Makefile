# test:
# 	make client
# 	./scripts/build
# -----------------------------------------------------------------------------
build:
	# ./bin/rebuild_db
	make build_client
	# make build_engine
	make build_server
build_client:
	cd ./src/mxyz-client && \
		cargo build --target wasm32-unknown-unknown # --release
	cd ./src && \
		wasm-bindgen --target web --out-dir "./mxyz-server/static/pkg" "./target/wasm32-unknown-unknown/debug/mxyz_client.wasm"
build_engine:
	# ./bin/rebuild_db
	cd ./src && \
		cargo build -p mxyz-engine # --release
build_server:
	# ./bin/rebuild_db
	cd ./src && \
		cargo build -p mxyz-server # --release
# -----------------------------------------------------------------------------
run:
	make db
	make build_client
	# make build_engine
	make run_server
run_engine:
	# ./bin/rebuild_db
	cd ./src && \
		cargo run -p mxyz-engine # --release
run_server:
	# ./bin/rebuild_db
	cd ./src && \
		cargo run -p mxyz-server # --release
# -----------------------------------------------------------------------------
dc:
	cd ./src/ && \
		docker-compose up # --build -d
db:
	# ln -s /opt/homebrew/opt/postgresql@14/lib/postgresql@14/libpq.5.dylib /usr/local/lib/libpq.5.dylib
	./bin/rebuild_db
	cd ./src/mxyz-database && diesel migration redo
clean:
	rm -r ./src/target/* 2> /dev/null
	rm ./src/Cargo.lock 2> /dev/null
	rm ./src/mxyz-server/static/pkg/* 2> /dev/null
stats:
	./bin/stats/fuse.py
	./bin/stats/show_nr_of_lines_in_proj.py
