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
    fn get_commands(&self) -> Vec<CommandInfo>;
    fn get_attributes(&self) -> Vec<AttributeInfo>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;
}


//pub type DevConstructor = Fn(&String) -> Box<Device>;

fn handle_one_message(sock: &mut zmq::Socket, dev: &mut Device) -> SpinResult<()> {
    let msg = try!(util::recv_message(sock));
    //println!("msg in dev: {:?}", msg);

    let mut req: pr::Request = try!(protobuf::parse_from_bytes(&msg[3]));
    let mut rsp = pr::Response::new();
    rsp.set_seqno(req.get_seqno());

    if req.has_exec() {
        let mut exec_cmd = req.take_exec();
        let arg = exec_cmd.take_value();
        rsp.mut_exec().set_cmd(exec_cmd.get_cmd().into());
        match dev.exec_cmd(exec_cmd.get_cmd(), arg.into()) {
            Ok(rval) => rsp.mut_exec().set_value(rval.into()),
            Err(err) => rsp.mut_exec().set_error(err.into_proto()),
        }
    } else if req.has_rattr() {
        let at_read = req.take_rattr();
        rsp.mut_rattr().set_attr(at_read.get_attr().into());
        match dev.read_attr(at_read.get_attr()) {
            Ok(rval) => rsp.mut_rattr().set_value(rval.into()),
            Err(err) => rsp.mut_rattr().set_error(err.into_proto()),
        }
    } else if req.has_wattr() {
        let mut at_write = req.take_wattr();
        let val = at_write.take_value();
        rsp.mut_wattr().set_attr(at_write.get_attr().into());
        if let Err(err) = dev.write_attr(at_write.get_attr(), val.into()) {
            rsp.mut_wattr().set_error(err.into_proto());
        }
    } else if req.has_qapi() {
        rsp.mut_qapi().set_cmd(RepeatedField::from_vec(dev.get_commands()));
        rsp.mut_qapi().set_attr(RepeatedField::from_vec(dev.get_attributes()));
    }

    let rsp = try!(rsp.write_to_bytes());
    try!(util::send_message(sock, &[&msg[0], &msg[1], &msg[2], &rsp]));
    Ok(())
}


pub fn run_device<'a>(mut sock: zmq::Socket, mut dev: Box<Device>) {
    let dev = dev.deref_mut();
    loop {
        if let Err(e) = handle_one_message(&mut sock, dev) {
            println!("error handling request: {:?}", e);
        }
    }
}
