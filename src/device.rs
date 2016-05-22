// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Device trait.

use std::ops::DerefMut;
use std::collections::HashMap;

use protobuf;
use protobuf::{Message, RepeatedField};
use zmq;

use spin_proto as pr;

use arg::*;
use config::DevProp;
use error::{SpinResult, spin_err};
use util;

pub type PropMap = HashMap<String, Value>;


pub trait Device : Sync + Send {
    fn init(&mut self);
    fn delete(&mut self);

    fn get_name(&self) -> &str;
    fn get_props(&self) -> &PropMap;
    fn mut_props(&mut self) -> &mut PropMap;

    fn get_cmd_descs(&self) -> Vec<CmdDesc>;
    fn get_attr_descs(&self) -> Vec<AttrDesc>;
    fn get_prop_descs(&self) -> Vec<PropDesc>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;

    fn init_props(&mut self, cfg_props: Vec<DevProp>) {
        let descs = self.get_prop_descs();
        let propmap = self.mut_props();
        'outer: for cfg_prop in cfg_props {
            for propdesc in &descs {
                if cfg_prop.name == propdesc.get_name() {
                    if let Some(value) = cfg_prop.value.convert(propdesc.get_field_type()) {
                        propmap.insert(cfg_prop.name, value);
                    }
                    continue 'outer;
                }
            }
        }
        for mut propdesc in descs {
            if !propmap.contains_key(propdesc.get_name()) {
                let defvalue = Value::new(propdesc.take_default());
                if let Some(value) = defvalue.convert(propdesc.get_field_type()) {
                    propmap.insert(propdesc.take_name(), value);
                }
            }
        }
    }

    fn get_prop(&mut self, prop: &str) -> SpinResult<Value> {
        match self.get_props().get(prop) {
            None => spin_err("PropertyError", "No such property"),
            Some(val) => Ok(val.clone())
        }
    }

    #[allow(unused_variables)]
    fn set_prop(&mut self, prop: &str, val: Value) -> SpinResult<()> {
        if !self.get_props().contains_key(prop) {
            return spin_err("PropertyError", "No such property");
        }
        // TODO: make this quicker
        let mut proptype = pr::DataType::Void;
        for propdesc in self.get_prop_descs() {
            if propdesc.get_name() == prop {
                proptype = propdesc.get_field_type();
                break;
            }
        }
        if let Some(val) = val.convert(proptype) {
            self.mut_props().insert(prop.to_owned(), val);
            Ok(())
        } else {
            spin_err("PropertyError", "Wrong property type")
        }
    }
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
                    dev.delete();
                    dev.init();
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
            rsp.set_cmds(RepeatedField::from_vec(dev.get_cmd_descs()));
            rsp.set_attrs(RepeatedField::from_vec(dev.get_attr_descs()));
            rsp.set_props(RepeatedField::from_vec(dev.get_prop_descs()));
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
     props [$($pname:ident => ($pdoc:expr, $ptype:expr, $pdef:expr)),*]) => {
        impl ::spin::device::Device for $clsname {
            fn init(&mut self) { $clsname::init(self) }

            fn delete(&mut self) { $clsname::delete(self) }

            fn get_name(&self) -> &str { &self.name }

            fn get_props(&self) -> &::spin::device::PropMap {
                &self.propmap
            }

            fn mut_props(&mut self) -> &mut ::spin::device::PropMap {
                &mut self.propmap
            }

            fn get_cmd_descs(&self) -> Vec<CmdDesc> {
                vec![$(cmd_info(stringify!($cname), $cdoc, $cintype, $couttype),)*]
            }

            fn get_attr_descs(&self) -> Vec<AttrDesc> {
                vec![$(attr_info(stringify!($aname), $adoc, $atype),)*]
            }

            fn get_prop_descs(&self) -> Vec<PropDesc> {
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
        }
    }
}
