// Spin RPC library, copyright 2015-2017 Georg Brandl.

fn main() {
    prost_build::compile_protos(&["msg.proto"], &["."]).unwrap();
}
