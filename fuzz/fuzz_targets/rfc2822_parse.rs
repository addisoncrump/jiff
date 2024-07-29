#![no_main]

use core::str::from_utf8;
use jiff::fmt::rfc2822::parse;
use libfuzzer_sys::{fuzz_target, Corpus};

fn do_fuzz(data: &[u8]) -> Corpus {
    if let Ok(src) = from_utf8(data) {
        let _ = parse(src);
        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { do_fuzz(data) });
