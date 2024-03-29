mod common;

use common::*;

use mc_legacy_formatting::{Color, Span, SpanIter, Styles};

pub fn spans_sc(start_char: char, s: &str) -> Vec<Span> {
    SpanIter::new(s).with_start_char(start_char).collect()
}

mod fake_codes {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn no_formatting_code() {
        let s = "this has no formatting codes";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("this has no formatting codes")]
        );
    }

    #[test]
    fn fake_code_at_start() {
        let s = "§this has no formatting codes";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("§this has no formatting codes")]
        );
    }

    #[test]
    fn fake_code_space_at_start() {
        let s = "§ this has no formatting codes";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("§ this has no formatting codes")]
        );
    }

    #[test]
    fn fake_code_at_end() {
        let s = "this has no formatting codes§";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("this has no formatting codes§")]
        );
    }

    #[test]
    fn fake_code_space_at_end() {
        let s = "this has no formatting codes §";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("this has no formatting codes §")]
        );
    }

    #[test]
    fn fake_code_middle() {
        let s = "this ha§s no formatting codes";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("this ha§s no formatting codes")]
        );
    }

    #[test]
    fn fake_code_space_middle() {
        let s = "this has no § formatting codes";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("this has no § formatting codes")]
        );
    }

    #[test]
    fn a_bunch_of_fakes() {
        let s = "§§§§§this has no format§ting codes§";
        assert_eq!(
            spans(s),
            vec![Span::new_plain("§§§§§this has no format§ting codes§")]
        );
    }
}

mod custom_start_char {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn using_ampersand() {
        let s = "&4this will be dark red";
        assert_eq!(
            spans_sc('&', s),
            vec![Span::new_styled(
                "this will be dark red",
                Color::DarkRed,
                Styles::empty()
            )]
        );
    }

    #[test]
    fn multiple_styles() {
        let s = "&1&e&d&lthis will be light purple and bold &o&a&e&a&mand this \
                will be green and strikethrough";
        assert_eq!(
            spans_sc('&', s),
            vec![
                Span::new_styled(
                    "this will be light purple and bold ",
                    Color::LightPurple,
                    Styles::BOLD
                ),
                Span::new_styled(
                    "and this will be green and strikethrough",
                    Color::Green,
                    Styles::STRIKETHROUGH
                )
            ]
        );
    }
}

#[test]
fn dark_red() {
    let s = "§4this will be dark red";
    assert_eq!(
        spans(s),
        vec![Span::new_styled(
            "this will be dark red",
            Color::DarkRed,
            Styles::empty()
        )]
    );
}

#[test]
fn dark_blue() {
    let s = "§1this will be dark blue";
    assert_eq!(
        spans(s),
        vec![Span::new_styled(
            "this will be dark blue",
            Color::DarkBlue,
            Styles::empty()
        )]
    );
}

#[test]
fn aqua() {
    let s = "§1§bthis will be aqua";
    assert_eq!(
        spans(s),
        vec![Span::new_styled(
            "this will be aqua",
            Color::Aqua,
            Styles::empty()
        )]
    );
}

#[test]
fn light_purple_and_bold() {
    let s = "§1§e§d§lthis will be light purple and bold";
    assert_eq!(
        spans(s),
        vec![Span::new_styled(
            "this will be light purple and bold",
            Color::LightPurple,
            Styles::BOLD
        )]
    );
}

#[test]
fn multiple_styles() {
    let s = "§1§e§d§lthis will be light purple and bold §o§a§e§a§mand this \
            will be green and strikethrough";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled(
                "this will be light purple and bold ",
                Color::LightPurple,
                Styles::BOLD
            ),
            Span::new_styled(
                "and this will be green and strikethrough",
                Color::Green,
                Styles::STRIKETHROUGH
            )
        ]
    );
}

#[test]
fn multiple_styles_no_colors() {
    let s = "§lthis will be bold §o§mand this will be bold, italic, and strikethrough";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled("this will be bold ", Color::White, Styles::BOLD),
            Span::new_styled(
                "and this will be bold, italic, and strikethrough",
                Color::White,
                Styles::BOLD | Styles::ITALIC | Styles::STRIKETHROUGH
            )
        ]
    );
}

#[test]
fn colors_and_styles_at_end() {
    let s = "basic stuff but then§o§a§e§a§m";
    assert_eq!(spans(s), vec![Span::new_plain("basic stuff but then")]);
}

#[test]
fn multiline_message() {
    let s = "§8Welcome to §6§lAmazing Minecraft Server\n§8§oYour hub for §d§op2w §8§ogameplay!";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled("Welcome to ", Color::DarkGray, Styles::empty()),
            Span::new_styled("Amazing Minecraft Server\n", Color::Gold, Styles::BOLD),
            Span::new_styled("Your hub for ", Color::DarkGray, Styles::ITALIC),
            Span::new_styled("p2w ", Color::LightPurple, Styles::ITALIC),
            Span::new_styled("gameplay!", Color::DarkGray, Styles::ITALIC)
        ]
    );
}

#[test]
fn yields_none_after_finish() {
    let s = "§lthis will be bold §o§mand this will be bold, italic, and strikethrough";
    let mut iter = SpanIter::new(s);

    while let Some(_) = iter.next() {}

    for _ in 0..20 {
        assert!(iter.next().is_none());
    }
}
