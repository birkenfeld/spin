// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Client library.

use std::io::Cursor;

use zmq;
use prost::Message;

use spin_proto::{Request, Response, NameValue, ApiDesc};
use spin_proto::request::ReqType;
use spin_proto::response::RspType;

use arg::{self, Value, FromValue};
use error::{SpinResult, Error, TIMEOUT_ERROR, API_ERROR};
use util;


pub struct Client {
    socket: zmq::Socket,
    local: bool,
    devname: Vec<u8>,
    seqno: u32,
    timeout: i64,  // in ms
}

impl Client {
    pub fn new(uri: &str) -> SpinResult<Client> {
        let sock = util::create_socket(zmq::REQ)?;
        let addr = util::DeviceAddress::parse_uri(uri)?;

        let (local, endpoint) = if Self::query_responder(&addr.devname)? {
            (true, format!("inproc://{}", addr.devname))
        } else if addr.use_db {
            (false, Self::query_db(&addr)?)
        } else {
            (false, addr.endpoint)
        };
        sock.connect(&endpoint)?;
        Ok(Client { socket:  sock,
                    local:   local,
                    devname: addr.devname.into_bytes(),
                    seqno:   0,
                    timeout: 1000, })
    }

    pub fn set_timeout(&mut self, timeout: i64) {
        self.timeout = timeout;
    }

    fn query_responder(addr: &str) -> SpinResult<bool> {
        let sock = util::create_socket(zmq::REQ)?;
        sock.connect("inproc://device_responder")?;
        sock.send(addr.as_bytes(), 0)?;
        let num = zmq::poll(&mut [sock.as_poll_item(zmq::POLLIN)], 50)?;
        if num == 0 {
            return Ok(false);
        }
        let reply = sock.recv_bytes(0)?;
        Ok(reply == b"1")
    }

    fn query_db(addr: &util::DeviceAddress) -> SpinResult<String> {
        let db_addr = format!("spin://{}/sys/spin/db", &addr.endpoint[6..]);
        let mut db_cl = Client::new(&db_addr)?;
        let srv_addr = db_cl.exec_cmd("Query", Value::from(addr.devname.clone()))?;
        srv_addr.extract()
    }

    fn do_request(&mut self, req_type: Option<ReqType>) -> SpinResult<Option<RspType>> {
        let req = Request {
            seqno: self.seqno,
            req_type: req_type,
        };
        self.seqno += 1;

        let mut buf = Vec::new();
        req.encode_length_delimited(&mut buf)?;
        if self.local {
            self.socket.send_multipart(&[b"", b"", &self.devname, &buf], 0)?;
        } else {
            self.socket.send_multipart(&[&self.devname, &buf], 0)?;
        }

        let num = zmq::poll(&mut [self.socket.as_poll_item(zmq::POLLIN)], self.timeout)?;
        if num == 0 {
            return spin_err!(TIMEOUT_ERROR, "no reply within client timeout");
        }
        let reply = util::recv_final_message_part(&mut self.socket)?;
        let rsp = Response::decode_length_delimited(&mut Cursor::new(reply))?;

        if rsp.seqno != req.seqno {
            return spin_err!(API_ERROR, "sequence numbers do not match");
        }
        if let Some(RspType::Error(e)) = rsp.rsp_type {
            Err(Error::new(e))
        } else {
            Ok(rsp.rsp_type)
        }
    }

    pub fn exec_cmd<I: Into<Value>>(&mut self, cmd: &str, arg: I) -> SpinResult<Value> {
        let args = NameValue { name: cmd.into(), value: arg.into().into_inner() };
        let rsp = self.do_request(Some(ReqType::ExecCmd(args)))?;
        if let Some(RspType::Value(value)) = rsp {
            Ok(Value::new(value))
        } else {
            spin_err!(API_ERROR, "wrong type of response received")
        }
    }

    pub fn exec_cmd_as<I, O>(&mut self, cmd: &str, arg: I) -> SpinResult<O>
        where I: Into<Value>, O: FromValue
    {
        self.exec_cmd(cmd, arg).and_then(FromValue::from_value)
    }

    pub fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
        let rsp = self.do_request(Some(ReqType::ReadAttr(attr.into())))?;
        if let Some(RspType::Value(value)) = rsp {
            Ok(Value::new(value))
        } else {
            spin_err!(API_ERROR, "wrong type of response received")
        }
    }

    pub fn read_attr_as<O: FromValue>(&mut self, attr: &str) -> SpinResult<O> {
        self.read_attr(attr).and_then(FromValue::from_value)
    }

    pub fn write_attr<I: Into<Value>>(&mut self, attr: &str, val: I) -> SpinResult<()> {
        let args = NameValue { name: attr.into(), value: val.into().into_inner() };
        let rsp = self.do_request(Some(ReqType::WriteAttr(args)))?;
        if rsp.is_none() {
            Ok(())
        } else {
            spin_err!(API_ERROR, "wrong type of response received")
        }
    }

    pub fn get_prop(&mut self, prop: &str) -> SpinResult<Value> {
        let rsp = self.do_request(Some(ReqType::GetProp(prop.into())))?;
        if let Some(RspType::Value(value)) = rsp {
            Ok(Value::new(value))
        } else {
            spin_err!(API_ERROR, "wrong type of response received")
        }
    }

    pub fn get_prop_as<O: FromValue>(&mut self, prop: &str) -> SpinResult<O> {
        self.get_prop(prop).and_then(FromValue::from_value)
    }

    pub fn set_prop<I: Into<Value>>(&mut self, prop: &str, val: I) -> SpinResult<()> {
        let args = NameValue { name: prop.into(), value: val.into().into_inner() };
        let rsp = self.do_request(Some(ReqType::SetProp(args)))?;
        if rsp.is_none() {
            Ok(())
        } else {
            spin_err!(API_ERROR, "wrong type of response received")
        }
    }

    pub fn query_api(&mut self) -> SpinResult<(Vec<arg::CmdDesc>, Vec<arg::AttrDesc>,
                                               Vec<arg::PropDesc>)> {
        let rsp = self.do_request(Some(ReqType::QueryApi(0)))?;
        if let Some(RspType::ApiDesc(ApiDesc { cmds, attrs, props})) = rsp {
            Ok((cmds, attrs, props))
        } else {
            spin_err!(API_ERROR, "wrong type of response received")
        }
    }

}
