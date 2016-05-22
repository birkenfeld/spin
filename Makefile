.PHONY: all proto demo

all:
	cargo build --release

proto:
	(cd src/proto; protoc --rust_out=. msg.proto)

demo:
	( cargo run --bin=spin_db -- db/1 -b :9999 & \
	DBPID=$$!; \
	sleep 0.2; \
	cargo run --bin=spin_echo echo/1 & \
	SRVPID=$$!; \
	sleep 0.2; \
	cargo run --example=echocl; \
	kill $$SRVPID; \
	kill $$DBPID )
