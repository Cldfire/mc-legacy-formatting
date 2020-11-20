use crate::{Color, Span, Styles};

/// A wrapper around [`Span`] that provides colored pretty-printing
///
/// # Examples
///
/// ```
/// use mc_legacy_formatting::{SpanExt, Span};
///
/// let s = "§4This will be dark red §oand italic";
/// s.span_iter().map(Span::wrap_colored).for_each(|s| print!("{}", s));
/// println!();
///
/// // Output will look close to what you'd see in Minecraft (ignoring the font difference)
/// ```
pub struct PrintSpanColored<'a>(Span<'a>);

impl<'a> From<Span<'a>> for PrintSpanColored<'a> {
    fn from(s: Span<'a>) -> Self {
        Self(s)
    }
}

impl<'a> core::fmt::Display for PrintSpanColored<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        fn apply_color_and_styles(s: &str, color: Color, styles: Styles) -> colored::ColoredString {
            use self::Styles as McStyles;
            use colored::*;

            let mut text = s.color(color);

            // TODO: handle random style

            if styles.contains(McStyles::BOLD) {
                text = text.bold();
            }

            if styles.contains(McStyles::STRIKETHROUGH) {
                text = text.strikethrough();
            }

            if styles.contains(McStyles::UNDERLINED) {
                text = text.underline();
            }

            if styles.contains(McStyles::ITALIC) {
                text = text.italic();
            }

            text
        }

        match self.0 {
            Span::Styled {
                text,
                color,
                styles,
            } => {
                let styled_text = apply_color_and_styles(text, color, styles);
                f.write_fmt(format_args!("{}", styled_text))
            }
            Span::Plain(_) => f.write_fmt(format_args!("{}", self.0)),
            Span::StrikethroughWhitespace {
                text,
                color,
                styles,
            } => (0..text.len()).try_for_each(|_| {
                f.write_fmt(format_args!(
                    "{}",
                    apply_color_and_styles("-", color, styles)
                ))
            }),
        }
    }
}

impl From<Color> for colored::Color {
    fn from(c: Color) -> Self {
        match c {
            Color::Black => colored::Color::Black,
            Color::DarkBlue => colored::Color::Blue,
            Color::DarkGreen => colored::Color::Green,
            Color::DarkAqua => colored::Color::Cyan,
            Color::DarkRed => colored::Color::Red,
            Color::DarkPurple => colored::Color::Magenta,
            Color::Gold => colored::Color::Yellow,
            Color::Gray => colored::Color::White,
            Color::DarkGray => colored::Color::BrightBlack,
            Color::Blue => colored::Color::BrightBlue,
            Color::Green => colored::Color::BrightGreen,
            Color::Aqua => colored::Color::BrightCyan,
            Color::Red => colored::Color::BrightRed,
            Color::LightPurple => colored::Color::BrightMagenta,
            Color::Yellow => colored::Color::BrightYellow,
            Color::White => colored::Color::BrightWhite,
        }
    }
}
