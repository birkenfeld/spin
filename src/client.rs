// Spin RPC library, copyright 2015, 2016 Georg Brandl.

//! Client library.

use protobuf;
use protobuf::Message;
use zmq;

use spin_proto::{Request, Response, ReqType, RespType};

use arg::{self, Value, FromValue};
use error::{SpinResult, Error, TIMEOUT_ERROR, API_ERROR};
use util;


pub struct Client {
    _context: zmq::Context,  // we have to keep it!
    socket: zmq::Socket,
    devname: Vec<u8>,
    seqno: u32,
    timeout: i64,  // in ms
}

impl Client {
    pub fn new(uri: &str) -> SpinResult<Client> {
        let mut ctx = zmq::Context::new();
        let mut sock = ctx.socket(zmq::REQ)?;
        sock.set_linger(0)?;  // no infinite wait for delivery on shutdown
        let addr = util::DeviceAddress::parse_uri(uri)?;
        let endpoint = if addr.use_db {
            Client::query_db(&addr)?
        } else {
            addr.endpoint
        };
        sock.connect(&endpoint)?;
        Ok(Client { _context: ctx,
                    socket:  sock,
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
        req.set_seqno(self.seqno);
        self.seqno += 1;

        let req_bytes = req.write_to_bytes()?;
        util::send_message(&mut self.socket, &[&self.devname, &req_bytes])?;

        let num = zmq::poll(&mut [self.socket.as_poll_item(zmq::POLLIN)], self.timeout)?;
        if num == 0 {
            return spin_err!(TIMEOUT_ERROR, "no reply within client timeout");
        }
        let reply = util::recv_message(&mut self.socket)?;

        let mut rsp: Response = protobuf::parse_from_bytes(&reply[1])?;

        if rsp.get_seqno() != req.get_seqno() {
            return spin_err!(API_ERROR, "sequence numbers do not match");
        }
        if rsp.get_rtype() == RespType::RespError {
            Err(Error::from_proto(rsp.take_error()))
        } else if rsp.get_rtype() != exp_type {
            spin_err!(API_ERROR, "wrong type of response received")
        } else {
            Ok(rsp)
        }
    }

    pub fn exec_cmd<I: Into<Value>>(&mut self, cmd: &str, arg: I) -> SpinResult<Value> {
        let mut req = Request::new();
        req.set_rtype(ReqType::ReqExecCmd);
        req.set_name(cmd.into());
        req.set_value(arg.into().into_inner());

        let mut rsp = self.do_request(req, RespType::RespValue)?;
        Ok(Value::new(rsp.take_value()))
    }

    pub fn exec_cmd_as<I, O>(&mut self, cmd: &str, arg: I) -> SpinResult<O>
        where I: Into<Value>, O: FromValue
    {
        self.exec_cmd(cmd, arg).and_then(FromValue::from_value)
    }

    pub fn read_attr(&mut self, attr: &str) -> SpinResult<Value> {
        let mut req = Request::new();
        req.set_rtype(ReqType::ReqReadAttr);
        req.set_name(attr.into());

        let mut rsp = self.do_request(req, RespType::RespValue)?;
        Ok(Value::new(rsp.take_value()))
    }

    pub fn read_attr_as<O: FromValue>(&mut self, attr: &str) -> SpinResult<O> {
        self.read_attr(attr).and_then(FromValue::from_value)
    }

    pub fn write_attr<I: Into<Value>>(&mut self, attr: &str, val: I) -> SpinResult<()> {
        let mut req = Request::new();
        req.set_rtype(ReqType::ReqWriteAttr);
        req.set_name(attr.into());
        req.set_value(val.into().into_inner());

        self.do_request(req, RespType::RespVoid)?;
        Ok(())
    }

    pub fn get_prop(&mut self, prop: &str) -> SpinResult<Value> {
        let mut req = Request::new();
        req.set_rtype(ReqType::ReqGetProp);
        req.set_name(prop.into());

        let mut rsp = self.do_request(req, RespType::RespValue)?;
        Ok(Value::new(rsp.take_value()))
    }

    pub fn get_prop_as<O: FromValue>(&mut self, prop: &str) -> SpinResult<O> {
        self.get_prop(prop).and_then(FromValue::from_value)
    }

    pub fn set_prop<I: Into<Value>>(&mut self, prop: &str, val: I) -> SpinResult<()> {
        let mut req = Request::new();
        req.set_rtype(ReqType::ReqSetProp);
        req.set_name(prop.into());
        req.set_value(val.into().into_inner());

        self.do_request(req, RespType::RespVoid)?;
        Ok(())
    }

    pub fn query_api(&mut self) -> SpinResult<(Vec<arg::CmdDesc>, Vec<arg::AttrDesc>,
                                               Vec<arg::PropDesc>)> {
        let mut req = Request::new();
        req.set_rtype(ReqType::ReqQueryAPI);

        let mut rsp = self.do_request(req, RespType::RespAPI)?;
        let cmds = rsp.take_cmds();
        let attrs = rsp.take_attrs();
        let props = rsp.take_props();
        Ok((cmds.to_vec(), attrs.to_vec(), props.to_vec()))
    }

}

impl Drop for Client {
    fn drop(&mut self) {
        let _ignored = self.socket.close();
    }
}
