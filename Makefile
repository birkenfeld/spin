.PHONY: all proto demo
MODE = release
MARG := $(if $(filter release,$(MODE)),--release,)

all:
	cargo build --release

proto:
	(cd src/proto; protoc --rust_out=. msg.proto)

demo:
	cargo build $(MARG) --bin=spin_db
	cargo build $(MARG) --bin=spin_echo
	cargo build $(MARG) --example=echocl
	@echo "Starting demo..."
	@( target/$(MODE)/spin_db db/1 -b :9999 & \
	   DBPID=$$!; \
	   sleep 0.2; \
	   target/$(MODE)/spin_echo echo/1 test/echo.srv & \
	   SRVPID=$$!; \
	   sleep 0.2; \
	   cargo run $(MARG) --example=echocl; \
	   kill $$SRVPID; \
	   kill $$DBPID )
