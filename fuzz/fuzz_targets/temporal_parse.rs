#![no_main]

use jiff::fmt::temporal;
use libfuzzer_sys::fuzz_target;

fn do_fuzz(data: &[u8]) {
    const TEMPORAL_PARSER: temporal::SpanParser = temporal::SpanParser::new();
    const TEMPORAL_PRINTER: temporal::SpanPrinter =
        temporal::SpanPrinter::new();
    if let Ok(first) = TEMPORAL_PARSER.parse_span(data) {
        let mut unparsed = Vec::with_capacity(data.len()); // get a good start at least
        TEMPORAL_PRINTER
            .print_span(&first, &mut unparsed)
            .expect("We parsed it, so we should be able to print it.");
        let second = TEMPORAL_PARSER
            .parse_span(unparsed)
            .expect("Should be able to parse a printed value.");
        assert_eq!(first, second, "Expected the initially parsed value to be equal to the value after printing and re-parsing.");
    }
}

fuzz_target!(|data: &[u8]| do_fuzz(data));
