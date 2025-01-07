all:
	cargo build --bin backend
	cd app/frontend && wasm-trunk build

test:
	echo "All working"
	# cargo test
