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
use error::{SpinResult, spin_err, API_ERROR};
use util;

pub type PropDescMap = HashMap<String, pr::PropDesc>;
pub type PropMap = HashMap<String, Value>;

/// Collects all the "internal" members every Device must have.
#[derive(Default)]
pub struct DeviceInner {
    pub propdesc: PropDescMap,
    pub props: PropMap,
}


pub trait Device : Sync + Send {
    fn init_caches(&mut self);
    fn init(&mut self) -> SpinResult<()>;
    fn delete(&mut self);

    fn get_name(&self) -> &str;
    fn get_props(&self) -> &PropMap;
    fn mut_props(&mut self) -> (&mut PropMap, &PropDescMap);

    fn query_cmd_descs(&self) -> Vec<CmdDesc>;
    fn query_attr_descs(&self) -> Vec<AttrDesc>;
    fn query_prop_descs(&self) -> Vec<PropDesc>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;

    fn init_props(&mut self, cfg_props: Vec<DevProp>) {
        let (propmap, descs) = self.mut_props();
        for cfg_prop in cfg_props {
            if let Some(desc) = descs.get(&cfg_prop.name) {
                if let Some(value) = cfg_prop.value.convert(desc.get_field_type()) {
                    propmap.insert(cfg_prop.name, value);
                }
            }
        }
        // TODO: ensure default value matches data type
        for (name, desc) in descs {
            if !propmap.contains_key(name) {
                let defvalue = Value::new(desc.get_default().clone());
                if let Some(value) = defvalue.convert(desc.get_field_type()) {
                    propmap.insert(name.to_owned(), value);
                }
            }
        }
    }

    fn get_prop(&mut self, prop: &str) -> SpinResult<Value> {
        match self.get_props().get(prop) {
            None => spin_err(API_ERROR, "No such property"),
            Some(val) => Ok(val.clone())
        }
    }

    #[allow(unused_variables)]
    fn set_prop(&mut self, prop: &str, val: Value) -> SpinResult<()> {
        let (propmap, descs) = self.mut_props();
        match descs.get(prop) {
            None => spin_err(API_ERROR, "No such property"),
            Some(desc) => {
                if let Some(val) = val.convert(desc.get_field_type()) {
                    propmap.insert(prop.to_owned(), val);
                    Ok(())
                } else {
                    spin_err(API_ERROR, "Wrong property type")
                }
            }
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
                    dev.init(); // TODO: this can fail now
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
            rsp.set_cmds(RepeatedField::from_vec(dev.query_cmd_descs()));
            rsp.set_attrs(RepeatedField::from_vec(dev.query_attr_descs()));
            rsp.set_props(RepeatedField::from_vec(dev.query_prop_descs()));
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
            fn init_caches(&mut self) {
                $(
                    self.inner.propdesc.insert(stringify!($pname).to_owned(),
                                               prop_info(stringify!($pname), $pdoc,
                                                         $ptype, Value::from($pdef)));
                )*
            }

            fn init(&mut self) -> ::spin::error::SpinResult<()> { $clsname::init(self) }

            fn delete(&mut self) { $clsname::delete(self) }

            fn get_name(&self) -> &str { &self.name }

            fn get_props(&self) -> &::spin::device::PropMap {
                &self.inner.props
            }

            fn mut_props(&mut self) -> (&mut ::spin::device::PropMap,
                                        &::spin::device::PropDescMap) {
                (&mut self.inner.props, &self.inner.propdesc)
            }

            fn query_cmd_descs(&self) -> Vec<CmdDesc> {
                vec![$(cmd_info(stringify!($cname), $cdoc, $cintype, $couttype),)*]
            }

            fn query_attr_descs(&self) -> Vec<AttrDesc> {
                vec![$(attr_info(stringify!($aname), $adoc, $atype),)*]
            }

            fn query_prop_descs(&self) -> Vec<PropDesc> {
                self.inner.propdesc.values().cloned().collect()
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: ::spin::arg::Value) -> ::spin::error::SpinResult<Value> {
                match cmd {
                    $(stringify!($cname) => self.$cfunc(arg.extract()?).map(Value::new),)*
                    _ => ::spin::error::spin_err(::spin::error::API_ERROR, "No such command"),
                }
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> ::spin::error::SpinResult<Value> {
                match attr {
                    $(stringify!($aname) => self.$arfunc().map(Value::new),)*
                    _ => ::spin::error::spin_err(::spin::error::API_ERROR, "No such attribute"),
                }
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: ::spin::arg::Value) -> ::spin::error::SpinResult<()> {
                match attr {
                    $(stringify!($aname) => self.$awfunc(val.extract()?),)*
                    _ => ::spin::error::spin_err(::spin::error::API_ERROR, "No such attribute"),
                }
            }
        }
    }
}
