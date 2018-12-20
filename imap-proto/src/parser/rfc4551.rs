//!
//! https://tools.ietf.org/html/rfc4551
//!
//! IMAP Extension for Conditional STORE Operation
//! or Quick Flag Changes Resynchronization
//!

// rustfmt doesn't do a very good job on nom parser invocations.
#![cfg_attr(rustfmt, rustfmt_skip)]
#![cfg_attr(feature = "cargo-clippy", allow(redundant_closure))]

use core::number_64;
use types::*;

// [3.6. HIGHESTMODSEQ Status Data Items](https://tools.ietf.org/html/rfc4551#section-3.6)
// The highest mod-sequence value of all messages in the mailbox.
named!(pub (crate) resp_text_code_highest_mod_seq<ResponseCode>, do_parse!(
    tag_s!("HIGHESTMODSEQ ") >>
    num: number_64 >>
    (ResponseCode::HighestModSeq(num))
));

named!(pub (crate) status_att_val<StatusAttribute>, do_parse!(
    tag_s!("HIGHESTMODSEQ ") >>
    // note the _64 here
    val: number_64 >>
    (StatusAttribute::HighestModSeq(val))
));

// aka `fetch-mod-resp`
named!(pub (crate) msg_att_mod_seq<AttributeValue>, do_parse!(
    tag_s!("MODSEQ (") >>
    num: number_64 >>
    tag_s!(")") >>
    (AttributeValue::ModSeq(num))
));