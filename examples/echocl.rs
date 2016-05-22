// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Echo client.

extern crate time;
extern crate spin;
extern crate spin_proto;

use spin::client;
use spin::arg;


fn mains() {
    let mut clnt = match client::Client::new("spindb://localhost:9999/test/dev/echo") {
        Err(e) => { println!("no connect: {:?}", e); return; },
        Ok(clnt) => clnt,
    };
    let val = arg::Value::new("Hello, world!");
    let exc: Result<String, _> = clnt.exec_cmd_as("Echo", val);
    println!("exc: {:?}", exc);
    let wat = clnt.write_attr("value", arg::Value::new(42.1));
    println!("wat: {:?}", wat);
    let rat: Result<f64, _> = clnt.read_attr_as("value");
    println!("rat: {:?}", rat);
    if let Ok(qap) = clnt.query_api() {
        let cnames: Vec<_> = qap.0.iter().map(|ci| ci.get_name()).collect();
        let anames: Vec<_> = qap.1.iter().map(|ai| ai.get_name()).collect();
        println!("qap: {:?} {:?}", cnames, anames);
    }
    let t1 = time::get_time();
    for _i in 0..10000 {
        let val = arg::Value::new("Hello, world!");
        clnt.exec_cmd("Echo", val).unwrap();
    }
    let t2 = time::get_time();
    println!("10000 calls -> {} us/call",
             ((t2 - t1).num_microseconds().unwrap() as f64) / 10000.);
}

fn main() {
    mains();
}
