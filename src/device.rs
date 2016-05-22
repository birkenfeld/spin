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
use error::SpinResult;
use util;


pub trait Device : Sync + Send {
    fn init_props(&mut self, HashMap<String, Value>);

    fn init_device(&mut self) -> SpinResult<()>;
    fn delete_device(&mut self);

    fn set_name(&mut self, String);
    fn get_name(&self) -> &str;

    fn query_cmd_descs(&self) -> Vec<CmdDesc>;
    fn query_attr_descs(&self) -> Vec<AttrDesc>;
    fn query_prop_descs(&self) -> Vec<PropDesc>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;
    fn get_prop(&mut self, prop: &str) -> SpinResult<Value>;
    fn set_prop(&mut self, prop: &str, val: Value) -> SpinResult<()>;
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
     $propstruct:ident,
     cmds  [$($cname:ident => ($cdoc:expr, $cintype:expr, $couttype:expr, $cfunc:ident)),*],
     attrs [$($aname:ident => ($adoc:expr, $atype:expr, $arfunc:ident, $awfunc:ident)),*],
     props [$($pname:ident => ($pdoc:expr, $ptype:expr, $prusttype:ty, $pdef:expr)),*]) => {
        #[derive(Default)]
        struct $propstruct {
            _name: String,
            _descriptions: Vec<::spin::arg::PropDesc>,
            _initialized: bool,
            $(
                $pname: $prusttype,
            )*
        }

        impl ::spin::device::Device for $clsname {

            fn init_device(&mut self) -> ::spin::error::SpinResult<()> {
                if let Err(err) = $clsname::init(self) {
                    error!("could not initialize {}: {}", self.get_name(), err);
                    return Err(err);
                }
                self.props._initialized = true;
                Ok(())
            }

            fn delete_device(&mut self) {
                self.props._initialized = false;
                $clsname::delete(self);
            }

            fn set_name(&mut self, name: String) { self.props._name = name; }
            fn get_name(&self) -> &str { &self.props._name }

            fn query_cmd_descs(&self) -> Vec<::spin::arg::CmdDesc> {
                vec![$(::spin::arg::cmd_info(stringify!($cname), $cdoc, $cintype, $couttype),)*]
            }

            fn query_attr_descs(&self) -> Vec<::spin::arg::AttrDesc> {
                vec![$(::spin::arg::attr_info(stringify!($aname), $adoc, $atype),)*]
            }

            fn query_prop_descs(&self) -> Vec<::spin::arg::PropDesc> {
                self.props._descriptions.clone()
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: ::spin::arg::Value)
                        -> ::spin::error::SpinResult<::spin::arg::Value>
            {
                if !self.props._initialized {
                    self.init_device()?;
                }
                match cmd {
                    $(stringify!($cname) => self.$cfunc(arg.extract()?).map(::spin::arg::Value::new),)*
                    _ => ::spin::error::spin_err(::spin::error::API_ERROR, "No such command"),
                }
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> ::spin::error::SpinResult<::spin::arg::Value> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                match attr {
                    $(stringify!($aname) => self.$arfunc().map(::spin::arg::Value::new),)*
                    _ => ::spin::error::spin_err(::spin::error::API_ERROR, "No such attribute"),
                }
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: ::spin::arg::Value)
                          -> ::spin::error::SpinResult<()>
            {
                if !self.props._initialized {
                    self.init_device()?;
                }
                match attr {
                    $(stringify!($aname) => self.$awfunc(val.extract()?),)*
                    _ => ::spin::error::spin_err(::spin::error::API_ERROR, "No such attribute"),
                }
            }

            #[allow(unused_variables, unused_mut)]
            fn init_props(&mut self, mut cfg_prop_map: ::std::collections::HashMap<String, ::spin::arg::Value>) {
                $(
                    self.props._descriptions.push(
                        ::spin::arg::prop_info(stringify!($pname), $pdoc, $ptype,
                                               ::spin::arg::Value::from($pdef)));
                    self.props.$pname = $pdef;
                    if let Some(cfg_value) = cfg_prop_map.remove(stringify!($pname)) {
                        if let Some(value) = cfg_value.convert($ptype) {
                            self.props.$pname = value.extract().unwrap();
                        }
                    }
                )*
            }

            #[allow(unused_variables)]
            fn get_prop(&mut self, prop: &str) -> ::spin::error::SpinResult<::spin::arg::Value> {
                $(
                    if prop == stringify!($pname) {
                        return Ok(::spin::arg::Value::new(self.props.$pname));
                    }
                )*;
                ::spin::error::spin_err(::spin::error::API_ERROR, "No such property")
            }

            #[allow(unused_variables)]
            fn set_prop(&mut self, prop: &str, val: ::spin::arg::Value)
                        -> ::spin::error::SpinResult<()>
            {
                $(
                    if prop == stringify!($pname) {
                        if let Some(val) = val.convert($ptype) {
                            self.props.$pname = val.extract().unwrap();
                            self.delete_device();
                            self.init_device()?;
                            return Ok(());
                        } else {
                            return ::spin::error::spin_err(::spin::error::ARG_ERROR,
                                                           &format!("Wrong property type, \
                                                                     expected {:?}", $ptype));
                        }
                    }
                )*;
                ::spin::error::spin_err(::spin::error::API_ERROR, "No such property")
            }
        }
    }
}
