// Spin RPC library, copyright 2015, 2016 Georg Brandl.
//
//! Device trait.

use std::ops::DerefMut;
use std::collections::HashMap;

use protobuf;
use protobuf::{Message, RepeatedField};
use zmq;

use spin_proto::{Request, Response, ReqType, RespType};

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

    let mut req: Request = protobuf::parse_from_bytes(&msg[3])?;
    let mut rsp = Response::new();
    rsp.set_seqno(req.get_seqno());

    match req.get_rtype() {
        ReqType::ReqPing => {

        }
        ReqType::ReqExecCmd => {
            let val = req.take_value();
            match dev.exec_cmd(req.get_name(), Value::new(val)) {
                Ok(val) => {
                    rsp.set_rtype(RespType::RespValue);
                    rsp.set_value(val.into_inner());
                }
                Err(err) => {
                    rsp.set_rtype(RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        ReqType::ReqReadAttr => {
            match dev.read_attr(req.get_name()) {
                Ok(val) => {
                    rsp.set_rtype(RespType::RespValue);
                    rsp.set_value(val.into_inner());
                }
                Err(err) => {
                    rsp.set_rtype(RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        ReqType::ReqWriteAttr => {
            let val = req.take_value();
            match dev.write_attr(req.get_name(), Value::new(val)) {
                Ok(_) => {
                    rsp.set_rtype(RespType::RespVoid);
                }
                Err(err) => {
                    rsp.set_rtype(RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        ReqType::ReqGetProp => {
            match dev.get_prop(req.get_name()) {
                Ok(val) => {
                    rsp.set_rtype(RespType::RespValue);
                    rsp.set_value(val.into_inner());
                }
                Err(err) => {
                    rsp.set_rtype(RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        ReqType::ReqSetProp => {
            let val = req.take_value();
            match dev.set_prop(req.get_name(), Value::new(val)) {
                Ok(_) => {
                    rsp.set_rtype(RespType::RespVoid);
                }
                Err(err) => {
                    rsp.set_rtype(RespType::RespError);
                    rsp.set_error(err.into_proto());
                }
            }
        }
        ReqType::ReqQueryAPI => {
            rsp.set_rtype(RespType::RespAPI);
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
    let req: Request = protobuf::parse_from_bytes(req)?;
    let mut rsp = Response::new();
    rsp.set_seqno(req.get_seqno());
    rsp.set_rtype(RespType::RespError);
    rsp.mut_error().set_reason(reason.into());
    rsp.mut_error().set_desc(desc.into());
    let rsp = rsp.write_to_bytes()?;
    Ok(rsp)
}

// Returns the Rust type for a given DataType type.
#[macro_export]
macro_rules! rust_type_for_data_type {
    (Void) => (());
    (Bool) => (bool);
    (Double) => (f64);
    (Float) => (f32);
    (Int32) => (i32);
    (Int64) => (i64);
    (UInt32) => (u32);
    (UInt64) => (u64);
    (String) => (String);
    (ByteArray) => (Vec<u8>);
    (BoolArray) => (Vec<bool>);
    (DoubleArray) => (Vec<f64>);
    (FloatArray) => (Vec<f32>);
    (Int32Array) => (Vec<i32>);
    (Int64Array) => (Vec<i64>);
    (UInt32Array) => (Vec<u32>);
    (UInt64Array) => (Vec<u64>);
    (StringArray) => (Vec<String>);
    (Int64StringArray) => ((Vec<i64>, Vec<String>));
    (DoubleStringArray) => ((Vec<f64>, Vec<String>));
}

#[macro_export]
macro_rules! spin_device_impl {
    ($clsname:ident,
     $propstruct:ident,
     cmds  [$($cname:ident => ($cdoc:expr, $cintype:ident, $couttype:ident, $cfunc:ident)),*],
     attrs [$($aname:ident => ($adoc:expr, $atype:ident, $arfunc:ident, $awfunc:ident)),*],
     props [$($pname:ident => ($pdoc:expr, $ptype:ident, $pdef:expr)),*]) => {
        #[derive(Default)]
        struct $propstruct {
            _name: String,
            _descriptions: Vec<$crate::arg::PropDesc>,
            _initialized: bool,
            $(
                $pname: rust_type_for_data_type!($ptype),
            )*
        }

        impl $crate::device::Device for $clsname {

            fn init_device(&mut self) -> $crate::error::SpinResult<()> {
                debug!("{}: initializing device", self.get_name());
                if let Err(err) = $clsname::init(self) {
                    error!("{}: could not initialize: {}", self.get_name(), err);
                    return Err(err);
                }
                self.props._initialized = true;
                Ok(())
            }

            fn delete_device(&mut self) {
                debug!("{}: deleting device", self.get_name());
                self.props._initialized = false;
                $clsname::delete(self);
            }

            fn set_name(&mut self, name: String) { self.props._name = name; }
            fn get_name(&self) -> &str { &self.props._name }

            fn query_cmd_descs(&self) -> Vec<$crate::arg::CmdDesc> {
                vec![$($crate::arg::cmd_info(stringify!($cname), $cdoc,
                                             $crate::arg::DataType::$cintype,
                                             $crate::arg::DataType::$couttype),)*]
            }

            fn query_attr_descs(&self) -> Vec<$crate::arg::AttrDesc> {
                vec![$($crate::arg::attr_info(stringify!($aname), $adoc,
                                              $crate::arg::DataType::$atype),)*]
            }

            fn query_prop_descs(&self) -> Vec<$crate::arg::PropDesc> {
                self.props._descriptions.clone()
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: $crate::arg::Value)
                        -> $crate::error::SpinResult<$crate::arg::Value>
            {
                if !self.props._initialized {
                    self.init_device()?;
                }
                debug!("{}: executing command {}({:?})", self.get_name(), cmd, arg);
                let res = match cmd {
                    $(stringify!($cname) => self.$cfunc(arg.extract()?).map($crate::arg::Value::from),)*
                    _ => spin_err!($crate::error::API_ERROR, "No such command"),
                };
                debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> $crate::error::SpinResult<$crate::arg::Value> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                debug!("{}: reading attribute {}", self.get_name(), attr);
                let res = match attr {
                    $(stringify!($aname) => self.$arfunc().map($crate::arg::Value::from),)*
                    _ => spin_err!($crate::error::API_ERROR, "No such attribute"),
                };
                debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: $crate::arg::Value)
                          -> $crate::error::SpinResult<()>
            {
                if !self.props._initialized {
                    self.init_device()?;
                }
                debug!("{}: writing attribute {} = {:?}", self.get_name(), attr, val);
                let res = match attr {
                    $(stringify!($aname) => self.$awfunc(val.extract()?),)*
                    _ => spin_err!($crate::error::API_ERROR, "No such attribute"),
                };
                debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables, unused_mut)]
            fn init_props(&mut self, mut cfg_prop_map: ::std::collections::HashMap<String, $crate::arg::Value>) {
                debug!("{}: init properties", self.get_name());
                $(
                    self.props._descriptions.push(
                        $crate::arg::prop_info(stringify!($pname), $pdoc,
                                               $crate::arg::DataType::$ptype,
                                               $crate::arg::Value::from($pdef)));
                    self.props.$pname = $pdef;
                    if let Some(cfg_value) = cfg_prop_map.remove(stringify!($pname)) {
                        if let Some(value) = cfg_value.convert($crate::arg::DataType::$ptype) {
                            self.props.$pname = value.extract().unwrap();
                            debug!("{}: property {} from config: {:?}", self.get_name(),
                                   stringify!($pname), self.props.$pname);
                        } else {
                            debug!("{}: property {} from default: {:?}", self.get_name(),
                                   stringify!($pname), self.props.$pname);
                        }
                    } else {
                        debug!("{}: property {} from default: {:?}", self.get_name(),
                               stringify!($pname), self.props.$pname);
                    }
                )*
            }

            #[allow(unused_variables)]
            fn get_prop(&mut self, prop: &str) -> $crate::error::SpinResult<$crate::arg::Value> {
                debug!("{}: get property {}", self.get_name(), prop);
                $(
                    if prop == stringify!($pname) {
                        return Ok($crate::arg::Value::from(self.props.$pname));
                    }
                )*;
                spin_err!($crate::error::API_ERROR, "No such property")
            }

            #[allow(unused_variables)]
            fn set_prop(&mut self, prop: &str, val: $crate::arg::Value)
                        -> $crate::error::SpinResult<()>
            {
                debug!("{}: set property {} = {:?}", self.get_name(), prop, val);
                $(
                    if prop == stringify!($pname) {
                        if let Some(val) = val.convert($crate::arg::DataType::$ptype) {
                            self.props.$pname = val.extract().unwrap();
                            self.delete_device();
                            self.init_device()?;
                            return Ok(());
                        } else {
                            return spin_err!(
                                $crate::error::ARG_ERROR,
                                &format!("Wrong property type, expected {:?}",
                                         $crate::arg::DataType::$ptype));
                        }
                    }
                )*;
                spin_err!($crate::error::API_ERROR, "No such property")
            }
        }
    }
}
