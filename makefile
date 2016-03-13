begin:
	${MAKE} -s all

all:
	cargo build
	./target/debug/program
