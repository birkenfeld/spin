// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Echo client.

extern crate time;
extern crate spin;

fn main() {
    // Connect
    let mut clnt = match spin::Client::new("spindb://localhost:9999/test/dev/echo") {
        Err(e) => { println!("connection failed: {:?}", e); return; },
        Ok(clnt) => clnt,
    };

    // Execute command
    let res = clnt.exec_cmd_as::<_, String>("Echo", "Hello!");
    println!("exec_cmd:   {:?}", res);

    // Read and write attribute
    let res = clnt.read_attr_as::<f64>("value");
    println!("read_attr:  {:?}", res);
    let res = clnt.write_attr("value", 5.5);
    println!("write_attr: {:?}", res);
    let res = clnt.read_attr_as::<f64>("value");
    println!("read_attr:  {:?}", res);

    // Get and set property
    let res = clnt.get_prop_as::<f64>("default_value");
    println!("get_prop:   {:?}", res);
    let res = clnt.set_prop("default_value", 67.2);
    println!("set_prop:   {:?}", res);
    // Attribute changed due to reinit after setting property
    let res = clnt.read_attr_as::<f64>("value");
    println!("read_attr:  {:?}", res);

    // Query API
    let res = clnt.query_api().unwrap();
    let cnames: Vec<_> = res.0.iter().map(|ci| ci.get_name()).collect();
    let anames: Vec<_> = res.1.iter().map(|ai| ai.get_name()).collect();
    let pnames: Vec<_> = res.2.iter().map(|pi| pi.get_name()).collect();
    println!("query_api:  {:?} {:?} {:?}", cnames, anames, pnames);

    // Time the simple Echo call
    let t1 = time::get_time();
    for _i in 0..10000 {
        clnt.exec_cmd("Echo", "Hello!").unwrap();
    }
    let t2 = time::get_time();
    println!("timing:     10000 calls -> {} us/call",
             ((t2 - t1).num_microseconds().unwrap() as f64) / 10000.);
}
