use std::str::CharIndices;

use bitflags::bitflags;

#[derive(Debug, Clone)]
pub struct SpanIter<'a> {
    buf: &'a str,
    chars: CharIndices<'a>,
    color: Color,
    styles: Styles,
    finished: bool,
}

impl<'a> SpanIter<'a> {
    pub fn new(buf: &'a str) -> Self {
        Self {
            buf,
            chars: buf.char_indices(),
            color: Color::White,
            styles: Styles::default(),
            finished: false,
        }
    }

    /// Update the currently stored color
    fn update_color(&mut self, color: Color) {
        self.color = color;
        // According to https://wiki.vg/Chat, using a color code resets the current
        // style
        self.styles = Styles::empty();
    }

    /// Insert `styles` into the currently stored styles
    fn update_styles(&mut self, styles: Styles) {
        if styles.contains(Styles::RESET) {
            self.color = Color::White;
            self.styles = Styles::empty();
        } else {
            self.styles.insert(styles);
        }
    }

    /// Make a `Span` based off the current state of the iterator
    ///
    /// The span will be from `start..end`
    fn make_span(&self, start: usize, end: usize) -> Span<'a> {
        if self.color == Color::White && self.styles.is_empty() {
            Span::Plain(&self.buf[start..end])
        } else {
            let text = &self.buf[start..end];

            // The vanilla client renders whitespace with `Styles::STRIKETHROUGH`
            // as a solid line. This replicates that behavior
            if text.chars().all(|c| c.is_ascii_whitespace()) {
                Span::StrikethroughWhitespace {
                    num_chars: text.len(),
                    color: self.color,
                    styles: self.styles,
                }
            } else {
                Span::Styled {
                    text,
                    color: self.color,
                    styles: self.styles,
                }
            }
        }
    }
}

/// Keeps track of the state for each iteration
#[derive(Debug, Copy, Clone)]
enum SpanIterState {
    GatheringStyles(GatheringStylesState),
    GatheringText(GatheringTextState),
}

/// In this state we are at the beginning of an iteration and we are looking to
/// handle any initial formatting codes
#[derive(Debug, Copy, Clone)]
enum GatheringStylesState {
    /// We're looking for our start char
    ExpectingStartChar,
    /// We've found our start char and are expecting a fmt code after it
    ExpectingFmtCode,
}

/// In this state we've encountered text unrelated to formatting, which means
/// the next valid fmt code we encounter ends this iteration
#[derive(Debug, Copy, Clone)]
enum GatheringTextState {
    /// We're waiting to find our start char
    WaitingForStartChar,
    /// We've found our start char and are expecting a fmt code after it
    ///
    /// If we find a valid fmt code in this state, we need to make a span, apply
    /// this last fmt code to our state, and end this iteration.
    ExpectingEndChar,
}

impl<'a> Iterator for SpanIter<'a> {
    type Item = Span<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let mut state = SpanIterState::GatheringStyles(GatheringStylesState::ExpectingStartChar);
        let mut span_start = None;
        let mut span_end = None;

        while let Some((idx, c)) = self.chars.next() {
            state = match state {
                SpanIterState::GatheringStyles(style_state) => match style_state {
                    GatheringStylesState::ExpectingStartChar => {
                        span_start = Some(idx);
                        match c {
                            '§' => SpanIterState::GatheringStyles(
                                GatheringStylesState::ExpectingFmtCode,
                            ),
                            _ => SpanIterState::GatheringText(
                                GatheringTextState::WaitingForStartChar,
                            ),
                        }
                    }
                    GatheringStylesState::ExpectingFmtCode => {
                        if let Some(color) = Color::from_char(c) {
                            self.update_color(color);
                            span_start = None;
                            SpanIterState::GatheringStyles(GatheringStylesState::ExpectingStartChar)
                        } else if let Some(style) = Styles::from_char(c) {
                            self.update_styles(style);
                            span_start = None;
                            SpanIterState::GatheringStyles(GatheringStylesState::ExpectingStartChar)
                        } else {
                            SpanIterState::GatheringText(GatheringTextState::WaitingForStartChar)
                        }
                    }
                },
                SpanIterState::GatheringText(text_state) => match text_state {
                    GatheringTextState::WaitingForStartChar => match c {
                        '§' => {
                            span_end = Some(idx);
                            SpanIterState::GatheringText(GatheringTextState::ExpectingEndChar)
                        }
                        _ => state,
                    },
                    GatheringTextState::ExpectingEndChar => {
                        // Note that we only end this iteration if we find a valid fmt code
                        //
                        // If we do, we make sure to apply it to our state so that we can
                        // pick up where we left off when the next iteration begins

                        if let Some(color) = Color::from_char(c) {
                            let span = self.make_span(span_start.unwrap(), span_end.unwrap());
                            self.update_color(color);
                            return Some(span);
                        } else if let Some(style) = Styles::from_char(c) {
                            let span = self.make_span(span_start.unwrap(), span_end.unwrap());
                            self.update_styles(style);
                            return Some(span);
                        } else {
                            span_end = None;
                            SpanIterState::GatheringText(GatheringTextState::WaitingForStartChar)
                        }
                    }
                },
            }
        }

        self.finished = true;
        span_start.map(|start| self.make_span(start, self.buf.len()))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Span<'a> {
    Styled {
        text: &'a str,
        color: Color,
        styles: Styles,
    },
    StrikethroughWhitespace {
        num_chars: usize,
        color: Color,
        styles: Styles,
    },
    Plain(&'a str),
}

impl<'a> std::fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Span::Styled { text, .. } => f.write_str(text),
            Span::StrikethroughWhitespace { num_chars, .. } => {
                (0..*num_chars).try_for_each(|_| f.write_str("-"))
            }
            Span::Plain(text) => f.write_str(text),
        }
    }
}

impl<'a> Span<'a> {
    pub fn new_plain(s: &'a str) -> Self {
        Span::Plain(s)
    }

    pub fn new_strikethrough_whitespace(num_chars: usize, color: Color, styles: Styles) -> Self {
        Span::StrikethroughWhitespace {
            num_chars,
            color,
            styles,
        }
    }

    pub fn new_styled(s: &'a str, color: Color, styles: Styles) -> Self {
        Span::Styled {
            text: s,
            color,
            styles,
        }
    }
}

pub struct PrintSpanColored<'a>(Span<'a>);

impl<'a> From<Span<'a>> for PrintSpanColored<'a> {
    fn from(s: Span<'a>) -> Self {
        Self(s)
    }
}

impl<'a> std::fmt::Display for PrintSpanColored<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn apply_color_and_styles(s: &str, color: Color, styles: Styles) -> colored::ColoredString {
            use self::Styles as McStyles;
            use colored::*;

            let mut text = s.color(color);

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
                num_chars,
                color,
                styles,
            } => (0..num_chars).try_for_each(|_| {
                f.write_fmt(format_args!(
                    "{}",
                    apply_color_and_styles("-", color, styles)
                ))
            }),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Color {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}

impl Default for Color {
    fn default() -> Self {
        Color::White
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

impl Color {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            '0' => Color::Black,
            '1' => Color::DarkBlue,
            '2' => Color::DarkGreen,
            '3' => Color::DarkAqua,
            '4' => Color::DarkRed,
            '5' => Color::DarkPurple,
            '6' => Color::Gold,
            '7' => Color::Gray,
            '8' => Color::DarkGray,
            '9' => Color::DarkBlue,
            // The vanilla client accepts lower or uppercase interchangeably
            'a' | 'A' => Color::Green,
            'b' | 'B' => Color::Aqua,
            'c' | 'C' => Color::Red,
            'd' | 'D' => Color::LightPurple,
            'e' | 'E' => Color::Yellow,
            'f' | 'F' => Color::White,
            _ => return None,
        })
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Styles: u32 {
        const RANDOM        = 0b00000001;
        const BOLD          = 0b00000010;
        const STRIKETHROUGH = 0b00000100;
        const UNDERLINED    = 0b00001000;
        const ITALIC        = 0b00010000;
        const RESET         = 0b00100000;
    }
}

impl Styles {
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            // The vanilla client accepts lower or uppercase interchangeably
            'k' | 'K' => Styles::RANDOM,
            'l' | 'L' => Styles::BOLD,
            'm' | 'M' => Styles::STRIKETHROUGH,
            'n' | 'N' => Styles::UNDERLINED,
            'o' | 'O' => Styles::ITALIC,
            'r' | 'R' => Styles::RESET,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn spans(s: &str) -> Vec<Span> {
        SpanIter::new(s).collect()
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
    fn real_motd() {
        let s = " §7§l<§a§l+§7§l>§8§l§m-----§8§l[ §a§lMine§7§lSuperior§a§l Network§8§l ]§8§l§m-----§7§l<§a§l+§7§l>\n\
                §a§l§n1.7-1.16 SUPPORT§r §7§l| §a§lSITE§7§l:§a§l§nwww.minesuperior.com";
        assert_eq!(
            spans(s),
            vec![
                Span::new_plain(" "),
                Span::new_styled("<", Color::Gray, Styles::BOLD),
                Span::new_styled("+", Color::Green, Styles::BOLD),
                Span::new_styled(">", Color::Gray, Styles::BOLD),
                Span::new_styled(
                    "-----",
                    Color::DarkGray,
                    Styles::BOLD | Styles::STRIKETHROUGH
                ),
                Span::new_styled("[ ", Color::DarkGray, Styles::BOLD),
                Span::new_styled("Mine", Color::Green, Styles::BOLD),
                Span::new_styled("Superior", Color::Gray, Styles::BOLD),
                Span::new_styled(" Network", Color::Green, Styles::BOLD),
                Span::new_styled(" ]", Color::DarkGray, Styles::BOLD),
                Span::new_styled(
                    "-----",
                    Color::DarkGray,
                    Styles::BOLD | Styles::STRIKETHROUGH
                ),
                Span::new_styled("<", Color::Gray, Styles::BOLD),
                Span::new_styled("+", Color::Green, Styles::BOLD),
                Span::new_styled(">\n", Color::Gray, Styles::BOLD),
                Span::new_styled(
                    "1.7-1.16 SUPPORT",
                    Color::Green,
                    Styles::BOLD | Styles::UNDERLINED
                ),
                Span::Plain(" "),
                Span::new_styled("| ", Color::Gray, Styles::BOLD),
                Span::new_styled("SITE", Color::Green, Styles::BOLD),
                Span::new_styled(":", Color::Gray, Styles::BOLD),
                Span::new_styled(
                    "www.minesuperior.com",
                    Color::Green,
                    Styles::BOLD | Styles::UNDERLINED
                )
            ]
        );
    }

    #[test]
    fn supports_uppercase_style_codes() {
        let s = "§5§m                  §6>§7§l§6§l>§6§l[§5§l§oPurple §8§l§oPrison§6§l]§6§l<§6<§5§m                     \
                    §R §7              (§4!§7) §e§lSERVER HAS §D§LRESET! §7(§4!§7)";
        assert_eq!(
            spans(s),
            vec![
                // The vanilla client renders whitespace with `Styles::STRIKETHROUGH`
                // as a solid line.
                Span::new_strikethrough_whitespace(18, Color::DarkPurple, Styles::STRIKETHROUGH),
                Span::new_styled(">", Color::Gold, Styles::empty()),
                Span::new_styled(">", Color::Gold, Styles::BOLD),
                Span::new_styled("[", Color::Gold, Styles::BOLD),
                Span::new_styled("Purple ", Color::DarkPurple, Styles::BOLD | Styles::ITALIC),
                Span::new_styled("Prison", Color::DarkGray, Styles::BOLD | Styles::ITALIC),
                Span::new_styled("]", Color::Gold, Styles::BOLD),
                Span::new_styled("<", Color::Gold, Styles::BOLD),
                Span::new_styled("<", Color::Gold, Styles::empty()),
                Span::new_strikethrough_whitespace(21, Color::DarkPurple, Styles::STRIKETHROUGH),
                Span::new_plain(" "),
                Span::new_styled("              (", Color::Gray, Styles::empty()),
                Span::new_styled("!", Color::DarkRed, Styles::empty()),
                Span::new_styled(") ", Color::Gray, Styles::empty()),
                Span::new_styled("SERVER HAS ", Color::Yellow, Styles::BOLD),
                Span::new_styled("RESET! ", Color::LightPurple, Styles::BOLD),
                Span::new_styled("(", Color::Gray, Styles::empty()),
                Span::new_styled("!", Color::DarkRed, Styles::empty()),
                Span::new_styled(")", Color::Gray, Styles::empty()),
            ]
        );
    }
}
