#![no_main]

use jiff::fmt::temporal;
use libfuzzer_sys::fuzz_target;

fn do_fuzz(data: &[u8]) {
    const TEMPORAL_PARSER: temporal::SpanParser = temporal::SpanParser::new();
    let _ = TEMPORAL_PARSER.parse_span(data);
}

fuzz_target!(|data: &[u8]| do_fuzz(data));
