//! A parser for Minecraft's [legacy formatting system][legacy_fmt], created
//! with careful attention to the quirks of the vanilla client's implementation.
//!
//! # Features
//!
//! * Iterator-based, non-allocating parser
//! * Supports `#![no_std]` usage (with `default-features` set to `false`)
//! * Implements the entire spec as well as vanilla client quirks (such as handling
//!   of whitespace with the `STRIKETHROUGH` style)
//! * Helpers for pretty-printing the parsed [`Span`]s to the terminal
//! * Support for parsing any start character for the formatting codes (vanilla
//!   uses `§` while many community tools use `&`)
//!
//! # Examples
//!
//! Using [`SpanIter`]:
//!
//! ```
//! use mc_legacy_formatting::{SpanExt, Span, Color, Styles};
//!
//! let s = "§4This will be dark red §oand italic";
//! let mut span_iter = s.span_iter();
//!
//! assert_eq!(span_iter.next().unwrap(), Span::new_styled("This will be dark red ", Color::DarkRed, Styles::empty()));
//! assert_eq!(span_iter.next().unwrap(), Span::new_styled("and italic", Color::DarkRed, Styles::ITALIC));
//! assert!(span_iter.next().is_none());
//! ```
//!
//! With a custom start character:
//!
//! ```
//! use mc_legacy_formatting::{SpanExt, Span, Color, Styles};
//!
//! let s = "&6It's a lot easier to type &b& &6than &b§";
//! let mut span_iter = s.span_iter().with_start_char('&');
//!
//! assert_eq!(span_iter.next().unwrap(), Span::new_styled("It's a lot easier to type ", Color::Gold, Styles::empty()));
//! assert_eq!(span_iter.next().unwrap(), Span::new_styled("& ", Color::Aqua, Styles::empty()));
//! assert_eq!(span_iter.next().unwrap(), Span::new_styled("than ", Color::Gold, Styles::empty()));
//! assert_eq!(span_iter.next().unwrap(), Span::new_styled("§", Color::Aqua, Styles::empty()));
//! assert!(span_iter.next().is_none());
//! ```
//!
//! [legacy_fmt]: https://wiki.vg/Chat#Colors

#![no_std]
#![deny(missing_docs)]
#![deny(unused_must_use)]

use core::str::CharIndices;

use bitflags::bitflags;

#[cfg(feature = "color-print")]
mod color_print;

#[cfg(feature = "color-print")]
pub use color_print::PrintSpanColored;

/// An extension trait that adds a method for creating a [`SpanIter`]
pub trait SpanExt {
    /// Produces a [`SpanIter`] from `&self`
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::{SpanExt, Span, Color, Styles};
    ///
    /// let s = "§4This will be dark red §oand italic";
    /// let mut span_iter = s.span_iter();
    ///
    /// assert_eq!(span_iter.next().unwrap(), Span::new_styled("This will be dark red ", Color::DarkRed, Styles::empty()));
    /// assert_eq!(span_iter.next().unwrap(), Span::new_styled("and italic", Color::DarkRed, Styles::ITALIC));
    /// assert!(span_iter.next().is_none());
    /// ```
    fn span_iter(&self) -> SpanIter;
}

impl<T: AsRef<str>> SpanExt for T {
    fn span_iter(&self) -> SpanIter {
        SpanIter::new(self.as_ref())
    }
}

/// An iterator that yields [`Span`]s from an input string.
///
/// # Examples
///
/// ```
/// use mc_legacy_formatting::{SpanIter, Span, Color, Styles};
///
/// let s = "§4This will be dark red §oand italic";
/// let mut span_iter = SpanIter::new(s);
///
/// assert_eq!(span_iter.next().unwrap(), Span::new_styled("This will be dark red ", Color::DarkRed, Styles::empty()));
/// assert_eq!(span_iter.next().unwrap(), Span::new_styled("and italic", Color::DarkRed, Styles::ITALIC));
/// assert!(span_iter.next().is_none());
/// ```
#[derive(Debug, Clone)]
pub struct SpanIter<'a> {
    buf: &'a str,
    chars: CharIndices<'a>,
    /// The character that indicates the beginning of a fmt code
    ///
    /// The vanilla client uses `§` for this, but community tooling often uses
    /// `&`, so we allow it to be configured
    start_char: char,
    color: Color,
    styles: Styles,
}

impl<'a> SpanIter<'a> {
    /// Create a new [`SpanIter`] to parse the given string
    pub fn new(s: &'a str) -> Self {
        Self {
            buf: s,
            chars: s.char_indices(),
            start_char: '§',
            color: Color::White,
            styles: Styles::default(),
        }
    }

    /// Set the start character used while parsing
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::{SpanIter, Span, Color, Styles};
    ///
    /// let s = "&6It's a lot easier to type &b& &6than &b§";
    /// let mut span_iter = SpanIter::new(s).with_start_char('&');
    ///
    /// assert_eq!(span_iter.next().unwrap(), Span::new_styled("It's a lot easier to type ", Color::Gold, Styles::empty()));
    /// assert_eq!(span_iter.next().unwrap(), Span::new_styled("& ", Color::Aqua, Styles::empty()));
    /// assert_eq!(span_iter.next().unwrap(), Span::new_styled("than ", Color::Gold, Styles::empty()));
    /// assert_eq!(span_iter.next().unwrap(), Span::new_styled("§", Color::Aqua, Styles::empty()));
    /// assert!(span_iter.next().is_none());
    /// ```
    pub fn with_start_char(mut self, c: char) -> Self {
        self.start_char = c;
        self
    }

    /// Set the start character used while parsing
    pub fn set_start_char(&mut self, c: char) {
        self.start_char = c;
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
        self.styles.insert(styles);
    }

    /// Should be called when encountering the `RESET` fmt code
    fn reset_styles(&mut self) {
        self.color = Color::White;
        self.styles = Styles::empty();
    }

    /// Make a [`Span`] based off the current state of the iterator
    ///
    /// The span will be from `start..end`
    fn make_span(&self, start: usize, end: usize) -> Span<'a> {
        if self.color == Color::White && self.styles.is_empty() {
            Span::Plain(&self.buf[start..end])
        } else {
            let text = &self.buf[start..end];

            // The vanilla client renders whitespace with `Styles::STRIKETHROUGH`
            // as a solid line. This replicates that behavior
            //
            // (Technically it does this by drawing a line over any text slice
            // with the `STRIKETHROUGH` style.)
            if text.chars().all(|c| c.is_ascii_whitespace())
                && self.styles.contains(Styles::STRIKETHROUGH)
            {
                Span::StrikethroughWhitespace {
                    text,
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
        use GatheringStylesState::*;
        use GatheringTextState::*;
        use SpanIterState::*;

        let mut state = GatheringStyles(ExpectingStartChar);
        let mut span_start = None;
        let mut span_end = None;

        while let Some((idx, c)) = self.chars.next() {
            state = match state {
                GatheringStyles(style_state) => match style_state {
                    ExpectingStartChar => {
                        span_start = Some(idx);
                        match c {
                            c if c == self.start_char => GatheringStyles(ExpectingFmtCode),
                            _ => GatheringText(WaitingForStartChar),
                        }
                    }
                    ExpectingFmtCode => {
                        if let Some(color) = Color::from_char(c) {
                            self.update_color(color);
                            span_start = None;
                            GatheringStyles(ExpectingStartChar)
                        } else if let Some(style) = Styles::from_char(c) {
                            self.update_styles(style);
                            span_start = None;
                            GatheringStyles(ExpectingStartChar)
                        } else if c == 'r' || c == 'R' {
                            // Handle the `RESET` fmt code

                            self.reset_styles();
                            span_start = None;
                            GatheringStyles(ExpectingStartChar)
                        } else {
                            GatheringText(WaitingForStartChar)
                        }
                    }
                },
                GatheringText(text_state) => match text_state {
                    WaitingForStartChar => match c {
                        c if c == self.start_char => {
                            span_end = Some(idx);
                            GatheringText(ExpectingEndChar)
                        }
                        _ => state,
                    },
                    ExpectingEndChar => {
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
                        } else if c == 'r' || c == 'R' {
                            // Handle the `RESET` fmt code

                            let span = self.make_span(span_start.unwrap(), span_end.unwrap());
                            self.reset_styles();
                            return Some(span);
                        } else {
                            span_end = None;
                            GatheringText(WaitingForStartChar)
                        }
                    }
                },
            }
        }

        span_start.map(|start| self.make_span(start, self.buf.len()))
    }
}

/// Text with an associated color and associated styles.
///
/// [`Span`] implements [`Display`](core::fmt::Display) and can be neatly printed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Span<'a> {
    /// A styled slice of text
    Styled {
        /// The styled text slice
        text: &'a str,
        /// The color of the text
        color: Color,
        /// Styles that should be applied to the text
        styles: Styles,
    },
    /// An unbroken sequence of whitespace that was given the
    /// [`STRIKETHROUGH`](Styles::STRIKETHROUGH) style.
    ///
    /// The vanilla client renders whitespace with the `STRIKETHROUGH` style
    /// as a solid line; this variant allows for replicating that behavior.
    StrikethroughWhitespace {
        /// The styled whitespace slice
        text: &'a str,
        /// The color of the whitespace (and therefore the line over it)
        color: Color,
        /// Styles applied to the whitespace (will contain at least
        /// [`STRIKETHROUGH`](Styles::STRIKETHROUGH))
        styles: Styles,
    },
    /// An unstyled slice of text
    ///
    /// This should be given a default style. The vanilla client
    /// would use [`Color::White`] and [`Styles::empty()`].
    Plain(&'a str),
}

impl core::fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            // TODO: handle random style
            Span::Styled { text, .. } => f.write_str(text),
            Span::StrikethroughWhitespace { text, .. } => {
                (0..text.len()).try_for_each(|_| f.write_str("-"))
            }
            Span::Plain(text) => f.write_str(text),
        }
    }
}

impl<'a> Span<'a> {
    /// Create a new [`Span::Plain`]
    pub fn new_plain(s: &'a str) -> Self {
        Span::Plain(s)
    }

    /// Create a new [`Span::StrikethroughWhitespace`]
    pub fn new_strikethrough_whitespace(s: &'a str, color: Color, styles: Styles) -> Self {
        Span::StrikethroughWhitespace {
            text: s,
            color,
            styles,
        }
    }

    /// Create a new [`Span::Styled`]
    pub fn new_styled(s: &'a str, color: Color, styles: Styles) -> Self {
        Span::Styled {
            text: s,
            color,
            styles,
        }
    }

    /// Wraps this [`Span`] in a type that enables colored printing
    #[cfg(feature = "color-print")]
    pub fn wrap_colored(self) -> PrintSpanColored<'a> {
        PrintSpanColored::from(self)
    }
}

/// Various colors that a [`Span`] can have.
///
/// See [the wiki.vg docs][colors] for specific information.
///
/// [colors]: https://wiki.vg/Chat#Colors
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[allow(missing_docs)]
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

impl Color {
    /// Map a `char` to a [`Color`].
    ///
    /// Returns [`None`] if `c` didn't map to a [`Color`].
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

    /// Get the correct foreground hex color string for a given color
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::Color;
    /// assert_eq!(Color::Aqua.foreground_hex_str(), "#55ffff");
    /// ```
    pub const fn foreground_hex_str(&self) -> &'static str {
        match self {
            Color::Black => "#000000",
            Color::DarkBlue => "#0000aa",
            Color::DarkGreen => "#00aa00",
            Color::DarkAqua => "#00aaaa",
            Color::DarkRed => "#aa0000",
            Color::DarkPurple => "#aa00aa",
            Color::Gold => "#ffaa00",
            Color::Gray => "#aaaaaa",
            Color::DarkGray => "#555555",
            Color::Blue => "#5555ff",
            Color::Green => "#55ff55",
            Color::Aqua => "#55ffff",
            Color::Red => "#ff5555",
            Color::LightPurple => "#ff55ff",
            Color::Yellow => "#ffff55",
            Color::White => "#ffffff",
        }
    }

    /// Get the correct background hex color string for a given color
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::Color;
    /// assert_eq!(Color::Aqua.background_hex_str(), "#153f3f");
    /// ```
    pub const fn background_hex_str(&self) -> &'static str {
        match self {
            Color::Black => "#000000",
            Color::DarkBlue => "#00002a",
            Color::DarkGreen => "#002a00",
            Color::DarkAqua => "#002a2a",
            Color::DarkRed => "#2a0000",
            Color::DarkPurple => "#2a002a",
            Color::Gold => "#2a2a00",
            Color::Gray => "#2a2a2a",
            Color::DarkGray => "#151515",
            Color::Blue => "#15153f",
            Color::Green => "#153f15",
            Color::Aqua => "#153f3f",
            Color::Red => "#3f1515",
            Color::LightPurple => "#3f153f",
            Color::Yellow => "#3f3f15",
            Color::White => "#3f3f3f",
        }
    }

    /// Get the correct foreground RGB color values for a given color
    ///
    /// Returns (red, green, blue)
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::Color;
    /// assert_eq!(Color::Aqua.foreground_rgb(), (85, 255, 255));
    /// ```
    pub const fn foreground_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::DarkBlue => (0, 0, 170),
            Color::DarkGreen => (0, 170, 0),
            Color::DarkAqua => (0, 170, 170),
            Color::DarkRed => (170, 0, 0),
            Color::DarkPurple => (170, 0, 170),
            Color::Gold => (255, 170, 0),
            Color::Gray => (170, 170, 170),
            Color::DarkGray => (85, 85, 85),
            Color::Blue => (85, 85, 255),
            Color::Green => (85, 255, 85),
            Color::Aqua => (85, 255, 255),
            Color::Red => (255, 85, 85),
            Color::LightPurple => (255, 85, 255),
            Color::Yellow => (255, 255, 85),
            Color::White => (255, 255, 255),
        }
    }

    /// Get the correct background RGB color values for a given color
    ///
    /// Returns (red, green, blue)
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::Color;
    /// assert_eq!(Color::Aqua.background_rgb(), (21, 63, 63));
    /// ```
    pub const fn background_rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Black => (0, 0, 0),
            Color::DarkBlue => (0, 0, 42),
            Color::DarkGreen => (0, 42, 0),
            Color::DarkAqua => (0, 42, 42),
            Color::DarkRed => (42, 0, 0),
            Color::DarkPurple => (42, 0, 42),
            Color::Gold => (42, 42, 0),
            Color::Gray => (42, 42, 42),
            Color::DarkGray => (21, 21, 21),
            Color::Blue => (21, 21, 63),
            Color::Green => (21, 63, 21),
            Color::Aqua => (21, 63, 63),
            Color::Red => (63, 21, 21),
            Color::LightPurple => (63, 21, 63),
            Color::Yellow => (63, 63, 21),
            Color::White => (63, 63, 63),
        }
    }
}

bitflags! {
    /// Styles that can be combined and applied to a [`Span`].
    ///
    /// The `RESET` flag is missing because the parser implemented in [`SpanIter`]
    /// takes care of it for you.
    ///
    /// See [wiki.vg's docs][styles] for detailed info about each style.
    ///
    /// # Examples
    ///
    /// ```
    /// use mc_legacy_formatting::Styles;
    /// let styles = Styles::BOLD | Styles::ITALIC | Styles::UNDERLINED;
    ///
    /// assert!(styles.contains(Styles::BOLD));
    /// assert!(!styles.contains(Styles::RANDOM));
    /// ```
    ///
    /// [styles]: https://wiki.vg/Chat#Styles
    #[derive(Default)]
    pub struct Styles: u32 {
        /// Signals that the `Span`'s text should be replaced with randomized
        /// characters at a constant interval
        const RANDOM        = 0b00000001;
        /// Signals that the `Span`'s text should be bold
        const BOLD          = 0b00000010;
        /// Signals that the `Span`'s text should be strikethrough
        const STRIKETHROUGH = 0b00000100;
        /// Signals that the `Span`'s text should be underlined
        const UNDERLINED    = 0b00001000;
        /// Signals that the `Span`'s text should be italic
        const ITALIC        = 0b00010000;
    }
}

impl Styles {
    /// Map a `char` to a [`Styles`] object.
    ///
    /// Returns [`None`] if `c` didn't map to a [`Styles`] object.
    pub fn from_char(c: char) -> Option<Self> {
        Some(match c {
            // The vanilla client accepts lower or uppercase interchangeably
            'k' | 'K' => Styles::RANDOM,
            'l' | 'L' => Styles::BOLD,
            'm' | 'M' => Styles::STRIKETHROUGH,
            'n' | 'N' => Styles::UNDERLINED,
            'o' | 'O' => Styles::ITALIC,
            _ => return None,
        })
    }
}
