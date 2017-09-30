// Spin RPC library, copyright 2015-2017 Georg Brandl.

spin_base_trait!(
    cmds = [
        Communicate => ("Send string, receive string.", String, String, cmd_communicate),
        Flush       => ("Flush all data.", (), (), cmd_flush),
        Read        => ("Read at most N chars.", u32, String, cmd_read),
        Write       => ("Write a string.", String, u32, cmd_write),
        ReadLine    => ("Read a line.", (), String, cmd_readline),
        WriteLine   => ("Write a line.", String, u32, cmd_writeline),
    ],
    attrs = [
        timeout     => ("Timeout for communication.", f64, read_timeout, write_timeout),
    ],
    props = [
        sol         => ("Start-of-line string.", String, ""),
        eol         => ("End-of-line string.", String, "\n"),
        timeout     => ("Initial timeout.", f64, 2.0),
    ],
);

pub mod default {
    use error::{SpinResult, MISSING_ERROR};
    use super::Base;

    pub fn cmd_communicate<B: Base + ?Sized>(this: &mut B, arg: String) -> SpinResult<String> {
        this.cmd_writeline(arg)?;
        this.cmd_readline(())
    }

    pub fn cmd_flush<B: Base + ?Sized>(_: &mut B, _: ()) -> SpinResult<()> {
        Ok(())
    }

    pub fn cmd_writeline<B: Base + ?Sized>(this: &mut B, mut arg: String) -> SpinResult<u32> {
        arg.push_str(&this.mut_props().eol);
        this.cmd_write(arg)
    }

    pub fn cmd_read<B: Base + ?Sized>(this: &mut B, _: u32) -> SpinResult<String> {
        spin_err!(MISSING_ERROR, &format!("cmd_read() for {} not implemented", this.get_name()))
    }

    pub fn cmd_write<B: Base + ?Sized>(this: &mut B, _: String) -> SpinResult<u32> {
        spin_err!(MISSING_ERROR, &format!("cmd_write() for {} not implemented", this.get_name()))
    }

    pub fn cmd_readline<B: Base + ?Sized>(this: &mut B, _: ()) -> SpinResult<String> {
        spin_err!(MISSING_ERROR, &format!("cmd_readline() for {} not implemented", this.get_name()))
    }

    pub fn read_timeout<B: Base + ?Sized>(this: &mut B) -> SpinResult<f64> {
        spin_err!(MISSING_ERROR, &format!("read_timeout() for {} not implemented", this.get_name()))
    }

    pub fn write_timeout<B: Base + ?Sized>(this: &mut B, _: f64) -> SpinResult<()> {
        spin_err!(MISSING_ERROR, &format!("write_timeout() for {} not implemented", this.get_name()))
    }
}
