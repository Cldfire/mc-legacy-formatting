use mc_legacy_formatting::{Span, SpanIter};

pub fn spans(s: &str) -> Vec<Span> {
    SpanIter::new(s).collect()
}

// There's a bug in rustc making it think this is unused
#[allow(dead_code)]
pub fn spans_sc(start_char: char, s: &str) -> Vec<Span> {
    SpanIter::new(s).with_start_char(start_char).collect()
}
