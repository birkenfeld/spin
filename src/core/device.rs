// Spin RPC library, copyright 2015-2020 Georg Brandl.

//! Device trait.

use std::ops::DerefMut;

use log::warn;
use fxhash::FxHashMap as HashMap;
use prost::Message;

use spin_proto::{ApiDesc, Error, NameValue, Request, Response};
use spin_proto::request::ReqType;
use spin_proto::response::RspType;

use crate::arg::{self, Value};
use crate::error::SpinResult;


pub trait Device: Send {
    fn init_props(&mut self, m: HashMap<String, Value>) -> SpinResult<()>;

    fn init_device(&mut self) -> SpinResult<()>;
    fn delete_device(&mut self);

    fn set_name(&mut self, n: String);
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


fn handle_one_message(sock: &mut zmq::Socket, dev: &mut dyn Device) -> SpinResult<()> {
    let msg = sock.recv_multipart(0)?;

    let req = Request::decode_length_delimited(msg[3].as_slice())?;
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
    sock.send_multipart(&[&msg[0][..], &[], &msg[2], &buf], 0)?;
    Ok(())
}


pub fn run_device(mut sock: zmq::Socket, mut dev: Box<dyn Device>) {
    let dev = dev.deref_mut();
    mlzlog::set_thread_prefix(format!("{}: ", dev.get_name()));
    loop {
        if let Err(e) = handle_one_message(&mut sock, dev) {
            warn!("error handling request: {:?}", e);
        }
    }
}


pub fn general_error_reply(reason: &str, desc: &str, req: &[u8]) -> SpinResult<Vec<u8>> {
    let req = Request::decode_length_delimited(req)?;
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

#[macro_export]
macro_rules! _data_type_ {
    ($ty:ty) => { <<$ty as $crate::validate::CanValidate>::Base
                    as $crate::arg::FromValue>::DATA_TYPE }
}

#[macro_export]
macro_rules! _rust_type_ {
    ($ty:ty) => { <$ty as $crate::validate::CanValidate>::Base }
}

#[macro_export]
macro_rules! spin_device_impl {
    ($clsname:ident,
     $propstruct:ident,
     bases = [$($base:ident),* $(,)*],
     cmds = [$($cname:ident => ($cdoc:expr, $cintype:ty, $couttype:ty, $cfunc:ident)),* $(,)*],
     attrs = [$($aname:ident => ($adoc:expr, $atype:ty, $arfunc:ident, $awfunc:ident)),* $(,)*],
     props = [$($pname:ident => ($pdoc:expr, $ptype:ty, $pdefault:expr)),* $(,)*] $(,)*) => {
        #[derive(Default)]
        struct $propstruct {
            _name: String,
            _descriptions: Vec<$crate::arg::PropDesc>,
            _initialized: bool,
            $(
                $base: $crate::base::$base::Props,
            )*
            $(
                $pname: <$ptype as $crate::validate::CanValidate>::Base,
            )*
        }

        impl $crate::device::Device for $clsname {

            fn init_device(&mut self) -> $crate::SpinResult<()> {
                log::debug!("initializing device");
                if let Err(err) = $clsname::init(self) {
                    log::error!("could not initialize: {}", err);
                    return Err(err);
                }
                self.props._initialized = true;
                Ok(())
            }

            fn delete_device(&mut self) {
                log::debug!("deleting device");
                self.props._initialized = false;
                $clsname::delete(self);
            }

            fn set_name(&mut self, name: String) { self.props._name = name; }
            fn get_name(&self) -> &str { &self.props._name }

            fn query_cmd_descs(&self) -> Vec<$crate::arg::CmdDesc> {
                let mut res = Vec::new();
                $(
                    res.append(&mut <Self as $crate::base::$base::Base>::query_cmd_descs());
                )*
                // XXX correct handling of overrides
                res.append(&mut vec![$($crate::arg::cmd_info(stringify!($cname), $cdoc,
                                                             $crate::_data_type_!($cintype),
                                                             $crate::_data_type_!($couttype)),)*]);
                res
            }

            fn query_attr_descs(&self) -> Vec<$crate::arg::AttrDesc> {
                let mut res = Vec::new();
                $(
                    res.append(&mut <Self as $crate::base::$base::Base>::query_attr_descs());
                )*
                // XXX correct handling of overrides
                    res.append(&mut vec![$($crate::arg::attr_info(stringify!($aname), $adoc,
                                                                  $crate::_data_type_!($atype)),)*]);
                res
            }

            fn query_prop_descs(&self) -> Vec<$crate::arg::PropDesc> {
                let mut res = Vec::new();
                $(
                    res.append(&mut <Self as $crate::base::$base::Base>::query_prop_descs(&self.props.$base));
                )*
                // XXX correct handling of overrides
                res.append(&mut self.props._descriptions.clone());
                res
            }

            #[allow(unused_variables)]
            fn exec_cmd(&mut self, cmd: &str, arg: $crate::Value)
                        -> $crate::SpinResult<$crate::Value> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                log::debug!("executing command {}({:?})", cmd, arg);
                let res = match cmd {
                    $(
                        stringify!($cname) => self.$cfunc(
                            <$cintype as $crate::validate::CanValidate>::validate(arg)?
                        ).map($crate::Value::from),
                    )*
                    _ => {
                        // try base trait methods
                        $(
                            if let Some(res) = $crate::base::$base::Base::exec_cmd(self, cmd, arg) {
                                return res;
                            }
                        )*
                        $crate::spin_err!($crate::error::API_ERROR, "No such command")
                    }
                };
                log::debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables)]
            fn read_attr(&mut self, attr: &str) -> $crate::SpinResult<$crate::Value> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                log::debug!("reading attribute {}", attr);
                let res = match attr {
                    $(stringify!($aname) => self.$arfunc().map($crate::Value::from),)*
                    _ => {
                        // try base trait attributes
                        $(
                            if let Some(res) = $crate::base::$base::Base::read_attr(self, attr) {
                                return res;
                            }
                        )*
                        $crate::spin_err!($crate::error::API_ERROR, "No such attribute")
                    }
                };
                log::debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables)]
            fn write_attr(&mut self, attr: &str, val: $crate::Value) -> $crate::SpinResult<()> {
                if !self.props._initialized {
                    self.init_device()?;
                }
                log::debug!("writing attribute {} = {:?}", attr, val);
                let res = match attr {
                    $(
                        stringify!($aname) => self.$awfunc(
                            <$atype as $crate::validate::CanValidate>::validate(val)?
                        ),
                    )*
                    _ => {
                        // try base trait attributes
                        $(
                            if let Some(res) = $crate::base::$base::Base::write_attr(self, attr, val) {
                                return res;
                            }
                        )*
                        $crate::spin_err!($crate::error::API_ERROR, "No such attribute")
                    }
                };
                log::debug!("   ... result: {:?}", res);
                res
            }

            #[allow(unused_variables, unused_mut)]
            fn init_props(&mut self, mut cfg_prop_map: ::fxhash::FxHashMap<String, $crate::Value>)
                -> $crate::SpinResult<()> {
                log::debug!("init properties");
                $(
                    self.props._descriptions.push(
                        $crate::arg::prop_info(stringify!($pname), $pdoc,
                                               $crate::_data_type_!($ptype),
                                               $crate::Value::from($pdefault)));
                    if let Some(cfg_value) = cfg_prop_map.remove(stringify!($pname)) {
                        if let Some(value) = cfg_value.convert($crate::_data_type_!($ptype)) {
                            self.props.$pname = <$ptype as $crate::validate::CanValidate>::validate(value)?;
                            log::debug!("property {} from config: {:?}",
                                        stringify!($pname), self.props.$pname);
                        } else {
                            return $crate::spin_err!($crate::error::CONFIG_ERROR,
                                                     &format!("Wrong configured type for {}, expected {:?}",
                                                              stringify!($pname), $crate::_data_type_!($ptype)));
                        }
                    } else {
                        self.props.$pname = $crate::validate::IntoDefault::into_default(&$pdefault)?.extract()?;
                        log::debug!("property {} from default: {:?}",
                                    stringify!($pname), self.props.$pname);
                    }
                )*
                $(
                    <Self as $crate::base::$base::Base>::init_props(&mut self.props.$base,
                                                                    &mut cfg_prop_map)?;
                )*
                Ok(())
            }

            #[allow(unused_variables)]
            fn get_prop(&mut self, prop: &str) -> $crate::SpinResult<$crate::Value> {
                log::debug!("get property {}", prop);
                $(
                    if prop == stringify!($pname) {
                        return Ok($crate::Value::from(self.props.$pname.clone()));
                    }
                )*;
                // try base trait properties
                $(
                    if let Some(res) = <Self as $crate::base::$base::Base>::get_prop(&self.props.$base, prop) {
                        return res;
                    }
                )*
                $crate::spin_err!($crate::error::API_ERROR, "No such property")
            }

            #[allow(unused_variables)]
            fn set_prop(&mut self, prop: &str, val: $crate::Value) -> $crate::SpinResult<()> {
                log::debug!("set property {} = {:?}", prop, val);
                $(
                    if prop == stringify!($pname) {
                        if let Some(val) = val.convert($crate::_data_type_!($ptype)) {
                            match <$ptype as $crate::validate::CanValidate>::validate(val) {
                                Ok(v) => self.props.$pname = v,
                                Err(e) => log::warn!("XXX property validation failure"),
                            }
                            self.delete_device();
                            self.init_device()?;
                            return Ok(());
                        } else {
                            return $crate::spin_err!(
                                $crate::error::ARG_ERROR,
                                &format!("Wrong property type, expected {:?}",
                                         $crate::_data_type_!($ptype))
                            );
                        }
                    }
                )*;
                // try base trait properties
                $(
                    if let Some(res) = <Self as $crate::base::$base::Base>::set_prop(
                        &mut self.props.$base, prop, val)
                    {
                        if res.is_ok() {
                            self.delete_device();
                            self.init_device()?;
                        }
                        return res;
                    }
                )*
                $crate::spin_err!($crate::error::API_ERROR, "No such property")
            }
        }

        $(
            impl $crate::base::$base::GetProps for $clsname {
                fn mut_props(&mut self) -> &mut $crate::base::$base::Props {
                    &mut self.props.$base
                }
            }
        )*
    }
}
