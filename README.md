# mc-legacy-formatting

[![Docs.rs docs](https://docs.rs/mc-legacy-formatting/badge.svg)](https://docs.rs/mc-legacy-formatting)
[![Crates.io version](https://img.shields.io/crates/v/mc-legacy-formatting.svg)](https://crates.io/crates/mc-legacy-formatting)
[![Crates.io downloads](https://img.shields.io/crates/d/mc-legacy-formatting.svg)](https://crates.io/crates/mc-legacy-formatting)
![CI](https://github.com/Cldfire/mc-legacy-formatting/workflows/CI/badge.svg)

A parser for Minecraft's [legacy formatting system][legacy_fmt], created
with careful attention to the quirks of the vanilla client's implementation.

## Features

* Iterator-based, non-allocating parser
* Supports the entire spec as well as vanilla client quirks (such as handling
  of whitespace with the `STRIKETHROUGH` style)
* Helpers for pretty-printing the parsed `Span`s to the terminal
* Support for parsing any start character for the formatting codes (vanilla
  uses `§` while many community tools use `&`)

## Examples

Using `SpanIter`:

```rust
use mc_legacy_formatting::{SpanIter, Span, Color, Styles};

let s = "§4This will be dark red §oand italic";
let mut span_iter = SpanIter::new(s);

assert_eq!(span_iter.next().unwrap(), Span::new_styled("This will be dark red ", Color::DarkRed, Styles::empty()));
assert_eq!(span_iter.next().unwrap(), Span::new_styled("and italic", Color::DarkRed, Styles::ITALIC));
assert!(span_iter.next().is_none());
```

With a custom start character:

```rust
use mc_legacy_formatting::{SpanIter, Span, Color, Styles};

let s = "&6It's a lot easier to type &b& &6than &b§";
let mut span_iter = SpanIter::new(s).with_start_char('&');

assert_eq!(span_iter.next().unwrap(), Span::new_styled("It's a lot easier to type ", Color::Gold, Styles::empty()));
assert_eq!(span_iter.next().unwrap(), Span::new_styled("& ", Color::Aqua, Styles::empty()));
assert_eq!(span_iter.next().unwrap(), Span::new_styled("than ", Color::Gold, Styles::empty()));
assert_eq!(span_iter.next().unwrap(), Span::new_styled("§", Color::Aqua, Styles::empty()));
assert!(span_iter.next().is_none());
```

[legacy_fmt]: https://wiki.vg/Chat#Colors

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
