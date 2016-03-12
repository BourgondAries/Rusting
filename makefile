all:
	cargo build --verbose
	./target/debug/program input
