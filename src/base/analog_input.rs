// Spin RPC library, copyright 2015-2017 Georg Brandl.

spin_base_trait!(
    cmds = [
    ],
    attrs = [
        value       => ("Main device value.", Double, read_value, write_value),
    ],
    props = [
    ],
);

pub mod default {
    use error::{SpinResult, MISSING_ERROR};
    use super::Base;

    pub fn read_value<B: Base + ?Sized>(this: &mut B) -> SpinResult<f64> {
        spin_err!(MISSING_ERROR, &format!("read_timeout() for {} not implemented", this.get_name()))
    }

    pub fn write_value<B: Base + ?Sized>(this: &mut B, _: f64) -> SpinResult<()> {
        spin_err!(MISSING_ERROR, &format!("write_timeout() for {} not implemented", this.get_name()))
    }
}
