// Spin RPC library, copyright 2015, 2016 Georg Brandl.
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
    fn get_name(&self) -> &str;
    fn get_cmds(&self) -> Vec<CmdDesc>;
    fn get_attrs(&self) -> Vec<AttrDesc>;
    fn get_props(&self) -> Vec<PropDesc>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;
    fn get_prop(&mut self, prop: &str) -> SpinResult<Value>;
    fn set_prop(&mut self, prop: &str, arg: Value) -> SpinResult<()>;
}


fn handle_one_message(sock: &mut zmq::Socket, dev: &mut Device) -> SpinResult<()> {
    let msg = util::recv_message(sock)?;
    debug!("msg in dev: {:?}", msg);

    let mut req: pr::Request = protobuf::parse_from_bytes(&msg[3])?;
    let mut rsp = pr::Response::new();
    rsp.set_seqno(req.get_seqno());

    match req.get_rtype() {
        pr::ReqType::ReqPing => {

        }
        pr::ReqType::ReqExecCmd => {
            let val = req.take_value();
            match dev.exec_cmd(req.get_name(), val.into()) {
                Ok(val) => {
                    rsp.set_rtype(pr::RespType::RespValue);
                    rsp.set_value(val.into());
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        pr::ReqType::ReqReadAttr => {
            match dev.read_attr(req.get_name()) {
                Ok(val) => {
                    rsp.set_rtype(pr::RespType::RespValue);
                    rsp.set_value(val.into());
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
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
        }
        pr::ReqType::ReqGetProp => {
            match dev.get_prop(req.get_name()) {
                Ok(val) => {
                    rsp.set_rtype(pr::RespType::RespValue);
                    rsp.set_value(val.into());
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        pr::ReqType::ReqSetProp => {
            let val = req.take_value();
            match dev.set_prop(req.get_name(), val.into()) {
                Ok(_) => {
                    rsp.set_rtype(pr::RespType::RespVoid);
                }
                Err(err) => {
                    rsp.set_rtype(pr::RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        pr::ReqType::ReqQueryAPI => {
            rsp.set_rtype(pr::RespType::RespAPI);
            rsp.set_cmds(RepeatedField::from_vec(dev.get_cmds()));
            rsp.set_attrs(RepeatedField::from_vec(dev.get_attrs()));
            rsp.set_props(RepeatedField::from_vec(dev.get_props()));
        }
    }

    let rsp = rsp.write_to_bytes()?;
    util::send_message(sock, &[&msg[0], &msg[1], &msg[2], &rsp])?;
    Ok(())
}


pub fn run_device(mut sock: zmq::Socket, mut dev: Box<Device>) {
    let dev = dev.deref_mut();
    loop {
        if let Err(e) = handle_one_message(&mut sock, dev) {
            warn!("{}: error handling request: {:?}", dev.get_name(), e);
        }
    }
}


pub fn general_error_reply(reason: &str, desc: &str, req: &[u8]) -> SpinResult<Vec<u8>> {
    let req: pr::Request = protobuf::parse_from_bytes(req)?;
    let mut rsp = pr::Response::new();
    rsp.set_seqno(req.get_seqno());
    rsp.set_rtype(pr::RespType::RespError);
    rsp.mut_error().set_reason(reason.into());
    rsp.mut_error().set_desc(desc.into());
    let rsp = rsp.write_to_bytes()?;
    Ok(rsp)
}


#[macro_export]
macro_rules! device_impl {
    ($clsname:ident,
     cmds  [$($cname:ident => ($cdoc:expr, $cintype:expr, $couttype:expr, $cfunc:ident)),*],
     attrs [$($aname:ident => ($adoc:expr, $atype:expr, $arfunc:ident, $awfunc:ident)),*],
     props [$($pname:ident => ($pdoc:expr, $ptype:expr, $pdef:expr, $prfunc:ident, $pwfunc:ident)),*]) => {
        impl ::spin::device::Device for $clsname {
            fn get_name(&self) -> &str { &self.name }

            fn get_cmds(&self) -> Vec<CmdDesc> {
                vec![$(cmd_info(stringify!($cname), $cdoc, $cintype, $couttype),)*]
            }

            fn get_attrs(&self) -> Vec<AttrDesc> {
                vec![$(attr_info(stringify!($aname), $adoc, $atype),)*]
            }

            fn get_props(&self) -> Vec<PropDesc> {
                // TODO: ensure default value matches data type
                vec![$(prop_info(stringify!($pname), $pdoc, $ptype, Value::from($pdef)),)*]
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value> {
                match cmd {
                    $(stringify!($cname) => self.$cfunc(arg),)*
                    _ => ::spin::error::spin_err("CommandError", "No such command"),
                }
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
                match attr {
                    $(stringify!($aname) => self.$arfunc(),)*
                    _ => ::spin::error::spin_err("AttributeError", "No such attribute"),
                }
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: Value) -> SpinResult<()> {
                match attr {
                    $(stringify!($aname) => self.$awfunc(val),)*
                    _ => ::spin::error::spin_err("AttributeError", "No such attribute"),
                }
            }

            #[allow(unused_variables)]
            fn get_prop(&mut self, prop: &str) -> SpinResult<Value> {
                match prop {
                    $(stringify!($pname) => self.$prfunc(),)*
                    _ => ::spin::error::spin_err("PropertyError", "No such property"),
                }
            }

            #[allow(unused_variables)]
            fn set_prop(&mut self, prop: &str, val: Value) -> SpinResult<()> {
                match prop {
                    $(stringify!($pname) => self.$pwfunc(val),)*
                    _ => ::spin::error::spin_err("PropertyError", "No such property"),
                }
                // TODO: should reinitialize after property change
            }
        }
    }
}
