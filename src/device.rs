// Spin RPC library, copyright 2015 Georg Brandl.
//
//! Device trait.

use std::ops::DerefMut;

use protobuf;
use protobuf::{Message, RepeatedField};
use zmq;

use spin_proto as pr;

use arg::*;
use util;
use error::SpinResult;


pub trait Device : Sync + Send {
    const CLSNAME: &'static str;

    fn get_name(&self) -> &str;
    fn get_commands(&self) -> Vec<CommandDesc>;
    fn get_attributes(&self) -> Vec<AttrDesc>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;
}


//pub type DevConstructor = Fn(&String) -> Box<Device>;

fn handle_one_message(sock: &mut zmq::Socket, dev: &mut Device) -> SpinResult<()> {
    let msg = try!(util::recv_message(sock));
    debug!("msg in dev: {:?}", msg);

    let mut req: pr::Request = try!(protobuf::parse_from_bytes(&msg[3]));
    let mut rsp = pr::Response::new();
    rsp.set_seqno(req.get_seqno());

    match req.get_rtype() {
        pr::ReqType::ReqPing => {

        },
        pr::ReqType::ReqExecCommand => {
            let val = req.take_value();
            match dev.exec_cmd(req.get_name(), val.into()) {
                Ok(rval) => {
                    rsp.set_rtype(pr::RespType::RespValue);
                    rsp.set_value(rval.into());
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        },
        pr::ReqType::ReqReadAttr => {
            match dev.read_attr(req.get_name()) {
                Ok(rval) => {
                    rsp.set_rtype(pr::RespType::RespValue);
                    rsp.set_value(rval.into());
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        },
        pr::ReqType::ReqWriteAttr => {
            let val = req.take_value();
            match dev.write_attr(req.get_name(), val.into()) {
                Ok(_) => {
                    rsp.set_rtype(pr::RespType::RespVoid);
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        },
        pr::ReqType::ReqQueryAPI => {
            rsp.set_rtype(pr::RespType::RespAPI);
            rsp.set_commands(RepeatedField::from_vec(dev.get_commands()));
            rsp.set_attrs(RepeatedField::from_vec(dev.get_attributes()));
        },
    }

    let rsp = try!(rsp.write_to_bytes());
    try!(util::send_message(sock, &[&msg[0], &msg[1], &msg[2], &rsp]));
    Ok(())
}


pub fn run_device<'a>(mut sock: zmq::Socket, mut dev: Box<Device>) {
    let dev = dev.deref_mut();
    loop {
        if let Err(e) = handle_one_message(&mut sock, dev) {
            warn!("{}: error handling request: {:?}", dev.get_name(), e);
        }
    }
}


pub fn general_error_reply(reason: &str, desc: &str, req: &Vec<u8>) -> SpinResult<Vec<u8>> {
    let req: pr::Request = try!(protobuf::parse_from_bytes(req));
    let mut rsp = pr::Response::new();
    rsp.set_seqno(req.get_seqno());
    rsp.set_rtype(pr::RespType::RespError);
    rsp.mut_error().set_reason(reason.into());
    rsp.mut_error().set_desc(desc.into());
    let rsp = try!(rsp.write_to_bytes());
    Ok(rsp)
}
