mod common;

use common::*;

use mc_legacy_formatting::{Color, Span, Styles};

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

#[test]
fn avoids_incorrect_whitespace_strikethrough() {
    let s = "§f§b§lMINE§6§lHEROES §7- §astore.mineheroes.net§a §2§l[75% Sale]\n\
            §b§lSKYBLOCK §f§l+ §2§lKRYPTON §f§lRESET! §f§l- §6§lNEW FALL CRATE";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled("MINE", Color::Aqua, Styles::BOLD),
            Span::new_styled("HEROES ", Color::Gold, Styles::BOLD),
            Span::new_styled("- ", Color::Gray, Styles::empty()),
            Span::new_styled("store.mineheroes.net", Color::Green, Styles::empty()),
            // A bug in the whitespace strikethrough handling was making this a
            // `Span::WhitespaceStrikethrough`
            Span::new_styled(" ", Color::Green, Styles::empty()),
            Span::new_styled("[75% Sale]\n", Color::DarkGreen, Styles::BOLD),
            Span::new_styled("SKYBLOCK ", Color::Aqua, Styles::BOLD),
            Span::new_styled("+ ", Color::White, Styles::BOLD),
            Span::new_styled("KRYPTON ", Color::DarkGreen, Styles::BOLD),
            Span::new_styled("RESET! ", Color::White, Styles::BOLD),
            Span::new_styled("- ", Color::White, Styles::BOLD),
            Span::new_styled("NEW FALL CRATE", Color::Gold, Styles::BOLD)
        ]
    );
}

#[test]
fn no_whitespace_strikethrough_involving_newline() {
    // Experimentation has shown the \n does not reset styles in any way in the vanilla
    // client (the vanilla client renders this far worse than we do, at least when used
    // in place of the player sample)
    let s = "§4§l§m⌜--------------------⌝\n   §4§lBLAZE§b-§6§lGAMING§b Network\n\n        \
            §bwww.mc-blaze.com\n            §8[§4116§7 /§4 1000§8]\n§4§l§m⌞--------------------⌟";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled(
                "⌜--------------------⌝\n   ",
                Color::DarkRed,
                Styles::BOLD | Styles::STRIKETHROUGH
            ),
            Span::new_styled("BLAZE", Color::DarkRed, Styles::BOLD),
            Span::new_styled("-", Color::Aqua, Styles::empty()),
            Span::new_styled("GAMING", Color::Gold, Styles::BOLD),
            Span::new_styled(" Network\n\n        ", Color::Aqua, Styles::empty()),
            Span::new_styled(
                "www.mc-blaze.com\n            ",
                Color::Aqua,
                Styles::empty()
            ),
            Span::new_styled("[", Color::DarkGray, Styles::empty()),
            Span::new_styled("116", Color::DarkRed, Styles::empty()),
            Span::new_styled(" /", Color::Gray, Styles::empty()),
            Span::new_styled(" 1000", Color::DarkRed, Styles::empty()),
            Span::new_styled("]\n", Color::DarkGray, Styles::empty()),
            Span::new_styled(
                "⌞--------------------⌟",
                Color::DarkRed,
                Styles::BOLD | Styles::STRIKETHROUGH
            )
        ]
    );
}
