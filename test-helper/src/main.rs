//! A little tool that accepts an input string with format codes and prints out
//! a vec of span constructors that can be copied into a test case

use dialoguer::Input;
use mc_legacy_formatting::{Span, SpanExt};

fn main() {
    let input = Input::<String>::new()
        .with_prompt("Input string (enclosed in quotes)")
        .interact()
        .unwrap();
    let s = &input[1..input.len() - 1];

    println!();
    println!("vec![");
    s.span_iter().for_each(|s| match s {
        Span::Styled {
            text,
            color,
            styles,
        } => println!(
            "\tSpan::new_styled(\"{}\", Color::{:?}, Styles::{}),",
            text,
            color,
            if styles.is_empty() {
                "empty()".into()
            } else {
                format!("{:?}", styles)
            }
        ),
        Span::StrikethroughWhitespace {
            num_chars,
            color,
            styles,
        } => println!(
            "\tSpan::new_strikethrough_whitespace({}, Color::{:?}, Styles::{}),",
            num_chars,
            color,
            if styles.is_empty() {
                "empty()".into()
            } else {
                format!("{:?}", styles)
            }
        ),
        Span::Plain(text) => println!("\tSpan::new_plain(\"{}\"),", text),
    });
    println!("];");
    println!();
}
