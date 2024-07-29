#![no_main]

use libfuzzer_sys::arbitrary::{Arbitrary, Unstructured};
use libfuzzer_sys::{arbitrary, fuzz_target};

use jiff::fmt::strtime::parse;

#[derive(Debug)]
struct Input<'a> {
    format: &'a [u8],
    input: &'a [u8],
}

impl<'a> Arbitrary<'a> for Input<'a> {
    fn arbitrary(u: &mut Unstructured<'a>) -> arbitrary::Result<Self> {
        let fmt_len: u8 = u.arbitrary()?;
        let in_len: u16 = u.arbitrary()?;
        let format = u.bytes(fmt_len as usize)?;
        let input = u.bytes(in_len as usize)?;
        Ok(Input { format, input })
    }

    fn arbitrary_take_rest(
        mut u: Unstructured<'a>,
    ) -> arbitrary::Result<Self> {
        let len: u8 = u.arbitrary()?;
        let _in_len: u16 = u.arbitrary()?; // ignored in take rest, but keep it consistent
        let format = u.bytes(len as usize)?;
        let input = u.take_rest();
        Ok(Input { format, input })
    }
}

fn do_fuzz(src: Input) {
    let _ = parse(src.format, src.input);
}

fuzz_target!(|data: Input<'_>| do_fuzz(data));
