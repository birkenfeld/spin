.PHONY: all release proto demo

all:
	cargo build

release:
	cargo build --release

proto:
	(cd src/proto; protoc --rust_out=. msg.proto)

MODE = release
MARG := $(if $(filter release,$(MODE)),--release,)

demo:
	cargo build $(MARG)
	cargo build $(MARG) --example=echocl
	@(  printf '    \033[01;32mStarting\033[0m DB...\n'; \
	    target/$(MODE)/spin_db -v db/1 --bind :9999 & \
	    DBPID=$$!; \
	    sleep 0.2; \
	    printf '    \033[01;32mStarting\033[0m server...\n'; \
	    target/$(MODE)/spin_srv echo/1 test/echo.srv & \
	    SRVPID=$$!; \
	    sleep 0.2; \
	    printf '    \033[01;32mStarting\033[0m client...\n'; \
	    target/$(MODE)/examples/echocl; \
	    printf '     \033[01;31mKilling\033[0m all processes...\n'; \
	    kill $$SRVPID; \
	    kill $$DBPID \
	)
