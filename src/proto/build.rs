// Spin RPC library, copyright 2015-2020 Georg Brandl.

fn main() {
    prost_build::compile_protos(&["msg.proto"], &["."]).unwrap();
}
