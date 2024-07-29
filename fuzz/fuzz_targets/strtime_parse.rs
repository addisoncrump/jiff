#![no_main]

use libfuzzer_sys::arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::{arbitrary, fuzz_target, Corpus};

use jiff::fmt::strtime::parse;

struct Input<'a> {
    format: &'a [u8],
    input: &'a [u8],
}

impl<'a> Arbitrary<'a> for Input<'a> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let len: u8 = u.arbitrary()?;
        let format = u.bytes(len as usize)?;
        let input = u.bytes(u.len())?;
        Ok(Input { format, input })
    }

    fn arbitrary_take_rest(
        mut u: Unstructured<'a>,
    ) -> arbitrary::Result<Self> {
        let len: u8 = u.arbitrary()?;
        let format = u.bytes(len as usize)?;
        let input = u.take_rest();
        Ok(Input { format, input })
    }
}

fn do_fuzz(data: &[u8]) -> Corpus {
    if let Ok(src) = Input::arbitrary_take_rest(Unstructured::new(data)) {
        let _ = parse(src.format, src.input);
        Corpus::Keep
    } else {
        Corpus::Reject
    }
}

fuzz_target!(|data: &[u8]| -> Corpus { do_fuzz(data) });
