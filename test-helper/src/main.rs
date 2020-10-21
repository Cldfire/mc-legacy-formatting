//! A little tool that accepts an input string with format codes and prints out
//! a vec of span constructors that can be copied into a test case

use dialoguer::Input;
use mc_legacy_formatting::{Span, SpanExt, Styles};

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
            "\tSpan::new_styled(\"{}\", Color::{:?}, {}),",
            text,
            color,
            handle_styles(styles)
        ),
        Span::StrikethroughWhitespace {
            num_chars,
            color,
            styles,
        } => println!(
            "\tSpan::new_strikethrough_whitespace({}, Color::{:?}, {}),",
            num_chars,
            color,
            handle_styles(styles)
        ),
        Span::Plain(text) => println!("\tSpan::new_plain(\"{}\"),", text),
    });
    println!("]");
    println!();
}

fn handle_styles(styles: Styles) -> String {
    if styles.is_empty() {
        "Styles::empty()".into()
    } else {
        let mut string = String::new();

        if styles.contains(Styles::RANDOM) {
            string.push_str("Styles::RANDOM | ");
        }

        if styles.contains(Styles::BOLD) {
            string.push_str("Styles::BOLD | ");
        }

        if styles.contains(Styles::STRIKETHROUGH) {
            string.push_str("Styles::STRIKETHROUGH | ");
        }

        if styles.contains(Styles::UNDERLINED) {
            string.push_str("Styles::UNDERLINED | ");
        }

        if styles.contains(Styles::ITALIC) {
            string.push_str("Styles::ITALIC | ");
        }

        string
            .strip_suffix(" | ")
            .map(|s| s.to_string())
            .unwrap_or_default()
    }
}
