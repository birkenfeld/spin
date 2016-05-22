// Spin RPC library, copyright 2015, 2016 Georg Brandl.

extern crate time;
extern crate spin;

fn main() {
    // Connect
    let mut clnt = match spin::Client::new("spindb://localhost:9999/test/cryo/dev") {
        Err(e) => { println!("connection failed: {:?}", e); return; },
        Ok(clnt) => clnt,
    };

    // Execute command
    const N: usize = 1000;
    let t1 = time::get_time();
    for _i in 0..N {
        let response = clnt.read_attr_as::<f64>("value").unwrap();
        //println!("value: {:?}", response);
    }
    let t2 = time::get_time();
    println!("timing:     {} calls -> {} us/call", N,
             ((t2 - t1).num_microseconds().unwrap() as f64) / (N as f64));
}
