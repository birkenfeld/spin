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
	@(  echo $$'     \e[01;32mStarting\e[0m DB...'; \
	    target/$(MODE)/spin_db db/1 -b :9999 & \
	    DBPID=$$!; \
	    sleep 0.2; \
	    echo $$'     \e[01;32mStarting\e[0m server...'; \
	    target/$(MODE)/spin_echo echo/1 test/echo.srv & \
	    SRVPID=$$!; \
	    sleep 0.2; \
	    echo $$'     \e[01;32mStarting\e[0m client...'; \
	    target/$(MODE)/examples/echocl; \
	    echo $$'     \e[01;31mKilling\e[0m processes...'; \
	    kill $$SRVPID; \
	    kill $$DBPID \
	)
