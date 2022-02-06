use eframe::egui::*;
use mc_legacy_formatting::{Color, Span, Styles};

pub fn render_mc_formatting(ui: &mut Ui, input: &str) {
    render_mc_formatting_items(
        ui,
        mc_legacy_formatting::SpanIter::new(input).with_start_char('&'),
    )
}

pub fn render_mc_formatting_items<'a>(ui: &mut Ui, items: impl Iterator<Item = Span<'a>>) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.set_row_height(ui.fonts()[TextStyle::Body].row_height());

        for item in items {
            span_ui(ui, item);
        }
    });
}

pub fn span_ui(ui: &mut Ui, span: Span<'_>) {
    match span {
        Span::Styled {
            text,
            color,
            styles,
        } => ui.add(label_from_style(text, color, styles)),
        Span::StrikethroughWhitespace {
            text,
            color,
            styles,
        } => ui.add(label_from_style(&"-".repeat(text.len()), color, styles)),
        Span::Plain(text) => ui.add(Label::new(RichText::new(text).color(Color32::WHITE))),
    };
}

pub fn label_from_style(text: &str, color: Color, styles: Styles) -> Label {
    let mut rich_text = RichText::new(text);

    if styles.contains(Styles::RANDOM) {
        // TODO: randomly generate this, animate it
        rich_text = RichText::new("1k4jkmnkjnqo");
    }

    if styles.contains(Styles::BOLD) {
        rich_text = rich_text.strong();
    }

    if styles.contains(Styles::ITALIC) {
        rich_text = rich_text.italics();
    }

    if styles.contains(Styles::STRIKETHROUGH) {
        rich_text = rich_text.strikethrough();
    }

    if styles.contains(Styles::UNDERLINED) {
        rich_text = rich_text.underline();
    }

    rich_text = rich_text.color(to_color_32(color.foreground_rgb()));
    // TODO: make background color more like MC
    // label = label.background_color(to_color_32(color.background_rgb()));

    // TODO: use MC font
    Label::new(rich_text)
}

pub fn to_color_32((r, g, b): (u8, u8, u8)) -> Color32 {
    Color32::from_rgb(r, g, b)
}
