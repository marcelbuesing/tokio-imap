// rustfmt doesn't do a very good job on nom parser invocations.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![cfg_attr(feature = "cargo-clippy", allow(redundant_closure))]

pub use self::rfc3501::*;

mod rfc3501;
mod rfc4551;