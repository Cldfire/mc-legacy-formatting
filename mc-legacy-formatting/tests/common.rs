use mc_legacy_formatting::{Span, SpanIter};

pub fn spans(s: &str) -> Vec<Span> {
    SpanIter::new(s).collect()
}
