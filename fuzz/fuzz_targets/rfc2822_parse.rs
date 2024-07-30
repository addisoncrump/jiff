#![cfg_attr(fuzzing, no_main)]
mod shim;

use libfuzzer_sys::fuzz_target;
use std::borrow::Cow;

use jiff::fmt::rfc2822;

fn do_fuzz(data: &[u8]) {
    const RFC2822_PARSER: rfc2822::DateTimeParser =
        rfc2822::DateTimeParser::new();
    const RFC2822_PRINTER: rfc2822::DateTimePrinter =
        rfc2822::DateTimePrinter::new();
    if let Ok(first) = RFC2822_PARSER.parse_zoned(data) {
        let mut unparsed = Vec::with_capacity(data.len()); // get a good start at least
        RFC2822_PRINTER
            .print_zoned(&first, &mut unparsed)
            .expect("We parsed it, so we should be able to print it");
        if let Ok(second) = RFC2822_PARSER.parse_zoned(&unparsed) {
            assert_eq!(first, second, "Expected the initially parsed value to be equal to the value after printing and re-parsing");
        } else if cfg!(not(feature = "relaxed")) {
            let unparsed_str = String::from_utf8_lossy(&unparsed);
            panic!(
                "Should be able to parse a printed value; failed at: `{}'{}",
                unparsed_str,
                if matches!(unparsed_str, Cow::Owned(_)) {
                    Cow::from(format!(" (lossy; actual bytes: {unparsed:?})"))
                } else {
                    Cow::from("")
                }
            );
        }
    }
}

fuzz_target!(|data: &[u8]| do_fuzz(data));

maybe_define_main!();
