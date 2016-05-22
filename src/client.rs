// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Client library.

use protobuf;
use protobuf::Message;
use zmq;

use spin_proto as pr;

use arg::*;
use error::{SpinResult, spin_err, Error};
use util;


pub struct Client {
    _context: zmq::Context,  // we have to keep it!
    socket: zmq::Socket,
    devname: Vec<u8>,
    seqno: u32,
}

impl Client {
    pub fn new(uri: &str) -> SpinResult<Client> {
        let mut ctx = zmq::Context::new();
        let mut sock = try!(ctx.socket(zmq::REQ));
        let addr = try!(util::DeviceAddress::parse_uri(uri));
        let endpoint = if addr.use_db {
            try!(Client::query_db(&addr))
        } else {
            addr.endpoint
        };
        println!("endpoint = {}", endpoint);
        try!(sock.connect(&endpoint));
        Ok(Client { _context: ctx,
                     socket:  sock,
                     devname: String::from(addr.devname).into_bytes(),
                     seqno:   0 })
    }

    fn query_db(addr: &util::DeviceAddress) -> SpinResult<String> {
        let db_addr = format!("spin://{}/sys/spin/db", &addr.endpoint[6..]);
        let mut db_cl = try!(Client::new(&db_addr));
        let srv_addr = try!(db_cl.exec_cmd("Query", Value::from(addr.devname.clone())));
        String::from_value(srv_addr)
    }

    fn do_request(&mut self, mut req: pr::Request, exp_type: pr::RespType) -> SpinResult<pr::Response> {
        req.set_seqno(self.seqno);
        self.seqno += 1;

        let req_bytes = try!(req.write_to_bytes());
        try!(util::send_message(&mut self.socket, &[&self.devname, &req_bytes]));

        let reply = try!(util::recv_message(&mut self.socket));

        let mut rsp: pr::Response = try!(protobuf::parse_from_bytes(&reply[1]));

        if rsp.get_seqno() != req.get_seqno() {
            return spin_err("APIError", "sequence numbers do not match");
        }
        if rsp.get_rtype() == pr::RespType::RespError {
            Err(Error::from_proto(rsp.take_error()))
        } else if rsp.get_rtype() != exp_type {
            spin_err("APIError", "wrong type of response received")
        } else {
            Ok(rsp)
        }
    }

    pub fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value> {
        let mut req = pr::Request::new();
        req.set_rtype(pr::ReqType::ReqExecCommand);
        req.set_name(cmd.into());
        req.set_value(arg.into());

        let mut rsp = try!(self.do_request(req, pr::RespType::RespValue));
        Ok(Value::from(rsp.take_value()))
    }

    pub fn exec_cmd_as<T: FromValue>(&mut self, cmd: &str, arg: Value) -> SpinResult<T> {
        self.exec_cmd(cmd, arg).and_then(FromValue::from_value)
    }

    pub fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
        let mut req = pr::Request::new();
        req.set_rtype(pr::ReqType::ReqReadAttr);
        req.set_name(attr.into());

        let mut rsp = try!(self.do_request(req, pr::RespType::RespValue));
        Ok(Value::from(rsp.take_value()))
    }

    pub fn read_attr_as<T: FromValue>(&mut self, attr: &str) -> SpinResult<T> {
        self.read_attr(attr).and_then(FromValue::from_value)
    }

    pub fn write_attr(&mut self, attr: &str, val: Value) -> SpinResult<()> {
        let mut req = pr::Request::new();
        req.set_rtype(pr::ReqType::ReqWriteAttr);
        req.set_name(attr.into());
        req.set_value(val.into());

        try!(self.do_request(req, pr::RespType::RespVoid));
        Ok(())
    }

    pub fn query_api(&mut self) -> SpinResult<(Vec<CommandDesc>, Vec<AttrDesc>)> {
        let mut req = pr::Request::new();
        req.set_rtype(pr::ReqType::ReqQueryAPI);

        let mut rsp = try!(self.do_request(req, pr::RespType::RespAPI));
        let cmds = rsp.take_commands();
        let attrs = rsp.take_attrs();
        Ok((cmds.to_vec(), attrs.to_vec()))
    }

}

impl Drop for Client {
    fn drop(&mut self) {
        let _ignored = self.socket.close();
    }
}
