use eframe::{
    egui::{self, color_picker::show_color, vec2, ScrollArea, TextEdit, TextStyle, Ui},
    epi,
};
use mc_legacy_formatting::{Color, Styles};

use crate::formatting::{label_from_style, render_mc_formatting, to_color_32};

// TODO: derive a thing to get an iterator over enum variants
const COLOR_GUIDE_ROWS: &[Color] = &[
    Color::Black,
    Color::DarkBlue,
    Color::DarkGreen,
    Color::DarkAqua,
    Color::DarkRed,
    Color::DarkPurple,
    Color::Gold,
    Color::Gray,
    Color::DarkGray,
    Color::Blue,
    Color::Green,
    Color::Aqua,
    Color::Red,
    Color::LightPurple,
    Color::Yellow,
    Color::White,
];

const STYLE_GUIDE_ROWS: &[Styles] = &[
    Styles::BOLD,
    Styles::ITALIC,
    Styles::RANDOM,
    Styles::STRIKETHROUGH,
    Styles::UNDERLINED,
    // represents reset for now
    Styles::empty(),
];

pub struct EditorApp {
    entered_text: String,
    preset: PresetText,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            entered_text: PresetText::default().text().to_string(),
            preset: Default::default(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum PresetText {
    Simple,
    HypixelMotd,
    MineheroesMotd,
}

impl Default for PresetText {
    fn default() -> Self {
        Self::Simple
    }
}

impl PresetText {
    fn text(&self) -> &'static str {
        match self {
            PresetText::Simple => "&6This is gold! &aThis is green!",
            PresetText::HypixelMotd => "             &aHypixel Network  &c[1.8-1.17]\n          &5&lSKYBLOCK CRYSTAL HOLLOWS!",
            PresetText::MineheroesMotd => "&f&b&lMINE&6&lHEROES &7- &astore.mineheroes.net&a &2&l[75% Sale]\n\
                &b&lSKYBLOCK &f&l+ &2&lKRYPTON &f&lRESET! &f&l- &6&lNEW FALL CRATE",
        }
    }

    fn menu_text(&self) -> &'static str {
        match self {
            PresetText::Simple => "Simple",
            PresetText::HypixelMotd => "Hypixel MOTD",
            PresetText::MineheroesMotd => "Mineheroes MOTD",
        }
    }
}

trait GuideRowItem {
    fn name(&self) -> &'static str;
    fn code(&self) -> &'static str;
    fn preview(&self, ui: &mut Ui);
}

impl GuideRowItem for Color {
    // TODO: add this to the lib
    fn name(&self) -> &'static str {
        match self {
            Color::Black => "Black",
            Color::DarkBlue => "Dark blue",
            Color::DarkGreen => "Dark green",
            Color::DarkAqua => "Dark cyan",
            Color::DarkRed => "Dark red",
            Color::DarkPurple => "Purple",
            Color::Gold => "Gold",
            Color::Gray => "Gray",
            Color::DarkGray => "Dark gray",
            Color::Blue => "Blue",
            Color::Green => "Bright green",
            Color::Aqua => "Cyan",
            Color::Red => "Red",
            Color::LightPurple => "Pink",
            Color::Yellow => "Yellow",
            Color::White => "White",
        }
    }

    // TODO: add this to the lib
    fn code(&self) -> &'static str {
        match self {
            Color::Black => "0",
            Color::DarkBlue => "1",
            Color::DarkGreen => "2",
            Color::DarkAqua => "3",
            Color::DarkRed => "4",
            Color::DarkPurple => "5",
            Color::Gold => "6",
            Color::Gray => "7",
            Color::DarkGray => "8",
            Color::Blue => "9",
            Color::Green => "a",
            Color::Aqua => "b",
            Color::Red => "c",
            Color::LightPurple => "d",
            Color::Yellow => "e",
            Color::White => "f",
        }
    }

    fn preview(&self, ui: &mut Ui) {
        show_color(ui, to_color_32(self.foreground_rgb()), vec2(36.0, 20.0));
    }
}

impl GuideRowItem for Styles {
    fn name(&self) -> &'static str {
        // TODO: add this to the lib
        if self.contains(Styles::BOLD) {
            "Bold"
        } else if self.contains(Styles::ITALIC) {
            "Italic"
        } else if self.contains(Styles::RANDOM) {
            "Random"
        } else if self.contains(Styles::STRIKETHROUGH) {
            "Strikethrough"
        } else if self.contains(Styles::UNDERLINED) {
            "Underlined"
        } else {
            // TODO: make reset an explicit style
            "Reset"
        }
    }

    // TODO: add this to the lib
    fn code(&self) -> &'static str {
        if self.contains(Styles::BOLD) {
            "l"
        } else if self.contains(Styles::ITALIC) {
            "o"
        } else if self.contains(Styles::RANDOM) {
            "k"
        } else if self.contains(Styles::STRIKETHROUGH) {
            "m"
        } else if self.contains(Styles::UNDERLINED) {
            "n"
        } else {
            // TODO: make reset an explicit style
            "r"
        }
    }

    fn preview(&self, ui: &mut Ui) {
        ui.add(label_from_style("text", Color::default(), *self));
    }
}

fn rows_for_guide_tables(ui: &mut Ui, data: &[impl GuideRowItem]) {
    for row in data {
        for col in 0..3 {
            match col {
                0 => row.preview(ui),
                1 => {
                    ui.scope(|ui| {
                        ui.style_mut().body_text_style = egui::TextStyle::Monospace;
                        ui.label(row.code());
                    });
                }
                2 => {
                    ui.label(row.name());
                }
                _ => unimplemented!(),
            }
        }
        ui.end_row();
    }
}

/// Show a button to switch to/from dark/light mode (globally).
fn dark_light_mode_switch(ui: &mut egui::Ui) {
    let style: egui::Style = (*ui.ctx().style()).clone();
    let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
    if let Some(visuals) = new_visuals {
        ui.ctx().set_visuals(visuals);
    }
}

impl epi::App for EditorApp {
    fn name(&self) -> &str {
        "mc-legacy-formatting editor"
    }

    fn max_size_points(&self) -> egui::Vec2 {
        vec2(160000.0, 160000.0)
    }

    fn update(&mut self, ctx: &egui::CtxRef, _: &mut epi::Frame<'_>) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("mc-legacy-formatting editor example");
            ui.hyperlink("https://github.com/Cldfire/mc-legacy-formatting");
            ui.add_space(10.0);
            egui::warn_if_debug_build(ui);

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Choose preset")
                    .selected_text(self.preset.menu_text())
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_value(&mut self.preset, PresetText::Simple, "Simple")
                            .clicked()
                        {
                            self.entered_text = PresetText::Simple.text().to_string();
                        }

                        if ui
                            .selectable_value(
                                &mut self.preset,
                                PresetText::HypixelMotd,
                                "Hypixel MOTD",
                            )
                            .clicked()
                        {
                            self.entered_text = PresetText::HypixelMotd.text().to_string();
                        };

                        if ui
                        .selectable_value(
                            &mut self.preset,
                            PresetText::MineheroesMotd,
                            "Mineheroes MOTD",
                        )
                        .clicked()
                    {
                        self.entered_text = PresetText::MineheroesMotd.text().to_string();
                    };
                    });
                dark_light_mode_switch(ui);
            });
            ui.separator();
            ui.columns(2, |columns| {
                ScrollArea::auto_sized()
                    .id_source("source")
                    .show(&mut columns[0], |ui| {
                        ui.add(
                            TextEdit::multiline(&mut self.entered_text)
                                .text_style(TextStyle::Monospace),
                        );
                    });
                ScrollArea::auto_sized()
                    .id_source("rendered")
                    .show(&mut columns[1], |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.set_row_height(ui.fonts()[TextStyle::Body].row_height());

                            render_mc_formatting(ui, &self.entered_text);
                        })
                    });
            });
            ui.collapsing("Guide", |ui| {
                ScrollArea::auto_sized()
                .id_source("guide")
                .show(ui, |ui| {
                    ui.label("Style text by prefixing strings with &{code}. Here are color codes that can be used:");
                    egui::Grid::new("color_guide_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            rows_for_guide_tables(ui, COLOR_GUIDE_ROWS);
                        });

                    ui.add_space(20.0);
                    ui.label("Text can also be styled using style codes:");
                    egui::Grid::new("style_guide_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            rows_for_guide_tables(ui, STYLE_GUIDE_ROWS);
                        });

                    ui.add_space(20.0);
                    ui.hyperlink_to("Learn more about Minecraft's legacy formatting codes.", "https://wiki.vg/Chat#Colors");
                });
            });
        });
    }
}
