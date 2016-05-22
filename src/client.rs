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
    pub fn new(addr: &str, devname: &str) -> Result<Client, zmq::Error> {
        let mut ctx = zmq::Context::new();
        let mut sock = try!(ctx.socket(zmq::REQ));
        try!(sock.connect(addr));
        Ok(Client { _context: ctx,
                     socket:  sock,
                     devname: String::from(devname).into_bytes(),
                     seqno:   0 })
    }

    fn do_request(&mut self, mut req: pr::Request) -> SpinResult<pr::Response> {
        req.set_seqno(self.seqno);
        self.seqno += 1;

        let req_bytes = try!(req.write_to_bytes());
        try!(util::send_message(&mut self.socket, &[&self.devname, &req_bytes]));

        let reply = try!(util::recv_message(&mut self.socket));

        let rsp: pr::Response = try!(protobuf::parse_from_bytes(&reply[1]));

        if rsp.get_seqno() != req.get_seqno() {
            return spin_err("SyncError", "sequence numbers out of order");
        }

        Ok(rsp)
    }

    pub fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value> {
        let mut req = pr::Request::new();
        req.mut_exec().set_cmd(cmd.into());
        req.mut_exec().set_value(arg.into());

        let mut rsp = try!(self.do_request(req));
        let mut ecr = rsp.take_exec();
        if ecr.has_error() {
            Err(Error::from_proto(ecr.take_error()))
        } else {
            Ok(Value::from(ecr.take_value()))
        }
    }

    pub fn exec_cmd_as<T: FromValue>(&mut self, cmd: &str, arg: Value) -> SpinResult<T> {
        self.exec_cmd(cmd, arg).and_then(FromValue::from_value)
    }

    pub fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
        let mut req = pr::Request::new();
        req.mut_rattr().set_attr(attr.into());

        let mut rsp = try!(self.do_request(req));
        let mut at_result = rsp.take_rattr();
        if at_result.has_error() {
            Err(Error::from_proto(at_result.take_error()))
        } else {
            Ok(Value::from(at_result.take_value()))
        }
    }

    pub fn read_attr_as<T: FromValue>(&mut self, attr: &str) -> SpinResult<T> {
        self.read_attr(attr).and_then(FromValue::from_value)
    }

    pub fn write_attr(&mut self, attr: &str, val: Value) -> SpinResult<()> {
        let mut req = pr::Request::new();
        req.mut_wattr().set_attr(attr.into());
        req.mut_wattr().set_value(val.into());

        let mut rsp = try!(self.do_request(req));
        let mut at_result = rsp.take_wattr();
        if at_result.has_error() {
            Err(Error::from_proto(at_result.take_error()))
        } else {
            Ok(())
        }
    }

    pub fn query_api(&mut self) -> SpinResult<(Vec<CommandInfo>, Vec<AttributeInfo>)> {
        let mut req = pr::Request::new();
        req.mut_qapi();

        let mut rsp = try!(self.do_request(req));

        let mut api = rsp.take_qapi();
        Ok((api.take_cmd().to_vec(), api.take_attr().to_vec()))
    }

}

impl Drop for Client {
    fn drop(&mut self) {
        let _ignored = self.socket.close();
    }
}
