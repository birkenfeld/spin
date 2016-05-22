// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Echo client.

extern crate spin;
extern crate spin_proto;

use spin::client;
use spin::arg;

fn mains() {
    let mut clnt = client::Client::new("tcp://localhost:1357", "test/dev/echo").unwrap();
    let val = arg::Value::new("Hello, world!");
    let exc: Result<String, _> = clnt.exec_cmd_as("Echo", val);
    println!("exc: {:?}", exc);
    let wat = clnt.write_attr("value", arg::Value::new(42.1));
    println!("wat: {:?}", wat);
    let rat: Result<f64, _> = clnt.read_attr_as("value");
    println!("rat: {:?}", rat);
    let qap = clnt.query_api().unwrap();
    let cnames: Vec<_> = qap.0.iter().map(|ci| ci.get_name()).collect();
    let anames: Vec<_> = qap.1.iter().map(|ai| ai.get_name()).collect();
    println!("qap: {:?} {:?}", cnames, anames);
}

fn main() {
    mains();
}
