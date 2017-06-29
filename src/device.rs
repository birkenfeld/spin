// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Device trait.

use std::io::Cursor;
use std::ops::DerefMut;

use fnv::FnvHashMap as HashMap;
use prost::Message;
use mlzlog;
use zmq;

use spin_proto::{Request, Response, NameValue, ApiDesc, Error};
use spin_proto::request::ReqType;
use spin_proto::response::RspType;

use arg::{self, Value};
use error::SpinResult;


pub trait Device : Send {
    fn init_props(&mut self, HashMap<String, Value>);

    fn init_device(&mut self) -> SpinResult<()>;
    fn delete_device(&mut self);

    fn set_name(&mut self, String);
    fn get_name(&self) -> &str;

    fn query_cmd_descs(&self) -> Vec<arg::CmdDesc>;
    fn query_attr_descs(&self) -> Vec<arg::AttrDesc>;
    fn query_prop_descs(&self) -> Vec<arg::PropDesc>;

    fn exec_cmd(&mut self, cmd: &str, arg: Value) -> SpinResult<Value>;
    fn read_attr(&mut self, attr: &str) -> SpinResult<Value>;
    fn write_attr(&mut self, attr: &str, arg: Value) -> SpinResult<()>;
    fn get_prop(&mut self, prop: &str) -> SpinResult<Value>;
    fn set_prop(&mut self, prop: &str, val: Value) -> SpinResult<()>;
}


fn handle_one_message(sock: &mut zmq::Socket, dev: &mut Device) -> SpinResult<()> {
    let msg = sock.recv_multipart(0)?;

    let req = Request::decode_length_delimited(&mut Cursor::new(&msg[3]))?;
    let mut rsp = Response {
        seqno: req.seqno,
        rsp_type: None,
    };

    match req.req_type {
        None => { }  // empty message == ok
        Some(ReqType::ExecCmd(NameValue { name, value })) => {
            match dev.exec_cmd(&name, Value::new(value)) {
                Ok(val)  => rsp.rsp_type = Some(RspType::Value(val.into_inner())),
                Err(err) => rsp.rsp_type = Some(RspType::Error(err.into_inner())),
            }
        }
        Some(ReqType::ReadAttr(name)) => {
            match dev.read_attr(&name) {
                Ok(val)  => rsp.rsp_type = Some(RspType::Value(val.into_inner())),
                Err(err) => rsp.rsp_type = Some(RspType::Error(err.into_inner())),
            }
        }
        Some(ReqType::WriteAttr(NameValue { name, value })) => {
            match dev.write_attr(&name, Value::new(value)) {
                Ok(_)    => { },
                Err(err) => rsp.rsp_type = Some(RspType::Error(err.into_inner())),
            }
        }
        Some(ReqType::GetProp(name)) => {
            match dev.get_prop(&name) {
                Ok(val)  => rsp.rsp_type = Some(RspType::Value(val.into_inner())),
                Err(err) => rsp.rsp_type = Some(RspType::Error(err.into_inner())),
            }
        }
        Some(ReqType::SetProp(NameValue { name, value })) => {
            match dev.set_prop(&name, Value::new(value)) {
                Ok(_)    => { },
                Err(err) => rsp.rsp_type = Some(RspType::Error(err.into_inner())),
            }
        }
        // XXX: instead, pack ApiDesc into a ByteArray and send as Value?
        Some(ReqType::QueryApi(_)) => {
            rsp.rsp_type = Some(RspType::ApiDesc(ApiDesc {
                cmds:  dev.query_cmd_descs(),
                attrs: dev.query_attr_descs(),
                props: dev.query_prop_descs(),
            }));
        }
    }

    let mut buf = Vec::new();
    rsp.encode_length_delimited(&mut buf)?;
    sock.send_multipart(&[&msg[0], &[], &msg[2], &buf], 0)?;
    Ok(())
}


pub fn run_device(mut sock: zmq::Socket, mut dev: Box<Device>) {
    let dev = dev.deref_mut();
    mlzlog::set_thread_prefix(format!("{}: ", dev.get_name()));
    loop {
        if let Err(e) = handle_one_message(&mut sock, dev) {
            warn!("error handling request: {:?}", e);
        }
    }
}


pub fn general_error_reply(reason: &str, desc: &str, req: &[u8]) -> SpinResult<Vec<u8>> {
    let req = Request::decode_length_delimited(&mut Cursor::new(req))?;
    let rsp = Response {
        seqno: req.seqno,
        rsp_type: Some(RspType::Error(Error {
            reason: reason.into(),
            desc: desc.into(),
            origin: "".into(),
        })),
    };
    let mut buf = Vec::new();
    rsp.encode_length_delimited(&mut buf)?;
    Ok(buf)
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
    (Uint32) => (u32);
    (Uint64) => (u64);
    (String) => (String);
    (Bytearray) => (Vec<u8>);
    (Boolarray) => (Vec<bool>);
    (Doublearray) => (Vec<f64>);
    (Floatarray) => (Vec<f32>);
    (Int32array) => (Vec<i32>);
    (Int64array) => (Vec<i64>);
    (UInt32array) => (Vec<u32>);
    (UInt64array) => (Vec<u64>);
    (Stringarray) => (Vec<String>);
    (Int64stringarray) => ((Vec<i64>, Vec<String>));
    (Doublestringarray) => ((Vec<f64>, Vec<String>));
}

#[macro_export]
macro_rules! spin_device_impl {
    ($clsname:ident,
     $propstruct:ident,
     cmds = [$($cname:ident => ($cdoc:expr, $cintype:ident, $couttype:ident, $cfunc:ident)),* $(,)*],
     attrs = [$($aname:ident => ($adoc:expr, $atype:ident, $arfunc:ident, $awfunc:ident)),* $(,)*],
     props = [$($pname:ident => ($pdoc:expr, $ptype:ident, $pdef:expr)),* $(,)*] $(,)*) => {
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

            fn init_device(&mut self) -> $crate::SpinResult<()> {
                debug!("initializing device");
                if let Err(err) = $clsname::init(self) {
                    error!("could not initialize: {}", err);
                    return Err(err);
                }
                self.props._initialized = true;
                Ok(())
            }

            fn delete_device(&mut self) {
                debug!("deleting device");
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
            fn exec_cmd(&mut self, cmd: &str, arg: $crate::Value)
                        -> $crate::SpinResult<$crate::Value> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                debug!("executing command {}({:?})", cmd, arg);
                let res = match cmd {
                    $(stringify!($cname) => self.$cfunc(arg.extract()?).map($crate::Value::from),)*
                    _ => spin_err!($crate::error::API_ERROR, "No such command"),
                };
                debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> $crate::SpinResult<$crate::Value> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                debug!("reading attribute {}", attr);
                let res = match attr {
                    $(stringify!($aname) => self.$arfunc().map($crate::Value::from),)*
                    _ => spin_err!($crate::error::API_ERROR, "No such attribute"),
                };
                debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: $crate::Value) -> $crate::SpinResult<()> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                debug!("writing attribute {} = {:?}", attr, val);
                let res = match attr {
                    $(stringify!($aname) => self.$awfunc(val.extract()?),)*
                    _ => spin_err!($crate::error::API_ERROR, "No such attribute"),
                };
                debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables, unused_mut)]
            fn init_props(&mut self, mut cfg_prop_map: ::fnv::FnvHashMap<String, $crate::Value>) {
                debug!("init properties");
                $(
                    self.props._descriptions.push(
                        $crate::arg::prop_info(stringify!($pname), $pdoc,
                                               $crate::arg::DataType::$ptype,
                                               $crate::Value::from($pdef)));
                    self.props.$pname = $pdef;
                    if let Some(cfg_value) = cfg_prop_map.remove(stringify!($pname)) {
                        if let Some(value) = cfg_value.convert($crate::arg::DataType::$ptype) {
                            self.props.$pname = value.extract().unwrap();
                            debug!("property {} from config: {:?}",
                                   stringify!($pname), self.props.$pname);
                        } else {
                            warn!("XXX property conversion failure");
                            debug!("property {} from default: {:?}",
                                   stringify!($pname), self.props.$pname);
                        }
                    } else {
                        debug!("property {} from default: {:?}",
                               stringify!($pname), self.props.$pname);
                    }
                )*
            }

            #[allow(unused_variables)]
            fn get_prop(&mut self, prop: &str) -> $crate::SpinResult<$crate::Value> {
                debug!("get property {}", prop);
                $(
                    if prop == stringify!($pname) {
                        return Ok($crate::Value::from(self.props.$pname.clone()));
                    }
                )*;
                spin_err!($crate::error::API_ERROR, "No such property")
            }

            #[allow(unused_variables)]
            fn set_prop(&mut self, prop: &str, val: $crate::Value) -> $crate::SpinResult<()> {
                debug!("set property {} = {:?}", prop, val);
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
