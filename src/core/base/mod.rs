// Spin RPC library, copyright 2015-2020 Georg Brandl.

#[macro_use]
mod macros;

pub mod string_io;
pub mod analog_input;

pub use self::string_io::Base as StringIO;
pub use self::analog_input::Base as AnalogInput;
