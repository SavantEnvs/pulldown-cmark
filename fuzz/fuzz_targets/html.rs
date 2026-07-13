#![no_main]
use libfuzzer_sys::fuzz_target;

use libfuzzer_sys::arbitrary::{self, Arbitrary};

#[derive(Debug, Arbitrary)]
struct FuzzingInput<'a> {
    options: u32,
    markdown: &'a str,
}

fuzz_target!(|data: FuzzingInput<'_>| {
    let opts = pulldown_cmark::Options::from_bits_truncate(data.options);

    let parser = pulldown_cmark::Parser::new_ext(data.markdown, opts);
    let mut output = String::new();

    pulldown_cmark::html::push_html(&mut output, parser);

    assert!(!output.contains('\0'));
    // assert!(!output.contains('\r'));
});
