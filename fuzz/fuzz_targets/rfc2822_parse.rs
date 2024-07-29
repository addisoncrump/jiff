#![no_main]

use libfuzzer_sys::fuzz_target;

use jiff::fmt::rfc2822;

fn do_fuzz(data: &[u8]) {
    const RFC2822_PARSER: rfc2822::DateTimeParser =
        rfc2822::DateTimeParser::new();
    let _ = RFC2822_PARSER.parse_zoned(data);
}

fuzz_target!(|data: &[u8]| do_fuzz(data));
