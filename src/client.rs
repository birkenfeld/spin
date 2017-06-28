// Spin RPC library, copyright 2015-2017 Georg Brandl.

//! Client library.

use std::io::Cursor;

use zmq;
use prost::Message;

use spin_proto::{Request, Response, ReqType, RespType};

use arg::{self, Value, FromValue};
use error::{SpinResult, Error, TIMEOUT_ERROR, API_ERROR};
use util;


pub struct Client {
    socket: zmq::Socket,
    devname: Vec<u8>,
    seqno: u32,
    timeout: i64,  // in ms
}

impl Client {
    pub fn new(uri: &str) -> SpinResult<Client> {
        let sock = util::create_socket(&zmq::Context::new(), zmq::REQ)?;
        let addr = util::DeviceAddress::parse_uri(uri)?;
        let endpoint = if addr.use_db {
            Client::query_db(&addr)?
        } else {
            addr.endpoint
        };
        sock.connect(&endpoint)?;
        Ok(Client { socket:  sock,
                    devname: String::from(addr.devname).into_bytes(),
                    seqno:   0,
                    timeout: 1000, })
    }

    pub fn set_timeout(&mut self, timeout: i64) {
        self.timeout = timeout;
    }

    fn query_db(addr: &util::DeviceAddress) -> SpinResult<String> {
        let db_addr = format!("spin://{}/sys/spin/db", &addr.endpoint[6..]);
        let mut db_cl = Client::new(&db_addr)?;
        let srv_addr = db_cl.exec_cmd("Query", Value::from(addr.devname.clone()))?;
        srv_addr.extract()
    }

    fn do_request(&mut self, mut req: Request, exp_type: RespType) -> SpinResult<Response> {
        req.seqno = self.seqno;
        self.seqno += 1;

        let mut buf = Vec::new();
        req.encode_length_delimited(&mut buf)?;
        util::send_message(&mut self.socket, &[&self.devname, &buf])?;

        let num = zmq::poll(&mut [self.socket.as_poll_item(zmq::POLLIN)], self.timeout)?;
        if num == 0 {
            return spin_err!(TIMEOUT_ERROR, "no reply within client timeout");
        }
        let reply = util::recv_message(&mut self.socket)?;

        let rsp: Response = Response::decode_length_delimited(&mut Cursor::new(&reply[1]))?;

        if rsp.seqno != req.seqno {
            return spin_err!(API_ERROR, "sequence numbers do not match");
        }
        if rsp.rtype() == Some(RespType::Error) {
            Err(Error::from_proto(rsp.error.unwrap()))
        } else if rsp.rtype() != Some(exp_type) {
            spin_err!(API_ERROR, "wrong type of response received")
        } else {
            Ok(rsp)
        }
    }

    pub fn exec_cmd<I: Into<Value>>(&mut self, cmd: &str, arg: I) -> SpinResult<Value> {
        let mut req = Request::default();
        req.set_rtype(ReqType::ExecCmd);
        req.name = Some(cmd.into());
        req.value = Some(arg.into().into_inner());

        let rsp = self.do_request(req, RespType::Value)?;
        Ok(Value::new(rsp.value.unwrap()))
    }

    pub fn exec_cmd_as<I, O>(&mut self, cmd: &str, arg: I) -> SpinResult<O>
        where I: Into<Value>, O: FromValue
    {
        self.exec_cmd(cmd, arg).and_then(FromValue::from_value)
    }

    pub fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
        let mut req = Request::default();
        req.set_rtype(ReqType::ReadAttr);
        req.name = Some(attr.into());

        let rsp = self.do_request(req, RespType::Value)?;
        Ok(Value::new(rsp.value.unwrap()))
    }

    pub fn read_attr_as<O: FromValue>(&mut self, attr: &str) -> SpinResult<O> {
        self.read_attr(attr).and_then(FromValue::from_value)
    }

    pub fn write_attr<I: Into<Value>>(&mut self, attr: &str, val: I) -> SpinResult<()> {
        let mut req = Request::default();
        req.set_rtype(ReqType::WriteAttr);
        req.name = Some(attr.into());
        req.value = Some(val.into().into_inner());

        self.do_request(req, RespType::Ok)?;
        Ok(())
    }

    pub fn get_prop(&mut self, prop: &str) -> SpinResult<Value> {
        let mut req = Request::default();
        req.set_rtype(ReqType::GetProp);
        req.name = Some(prop.into());

        let rsp = self.do_request(req, RespType::Value)?;
        Ok(Value::new(rsp.value.unwrap()))
    }

    pub fn get_prop_as<O: FromValue>(&mut self, prop: &str) -> SpinResult<O> {
        self.get_prop(prop).and_then(FromValue::from_value)
    }

    pub fn set_prop<I: Into<Value>>(&mut self, prop: &str, val: I) -> SpinResult<()> {
        let mut req = Request::default();
        req.set_rtype(ReqType::SetProp);
        req.name = Some(prop.into());
        req.value = Some(val.into().into_inner());

        self.do_request(req, RespType::Ok)?;
        Ok(())
    }

    pub fn query_api(&mut self) -> SpinResult<(Vec<arg::CmdDesc>, Vec<arg::AttrDesc>,
                                               Vec<arg::PropDesc>)> {
        let mut req = Request::default();
        req.set_rtype(ReqType::QueryApi);

        let rsp = self.do_request(req, RespType::Api)?;
        let cmds = rsp.cmds;
        let attrs = rsp.attrs;
        let props = rsp.props;
        Ok((cmds, attrs, props))
    }

}
