mod common;

use common::*;

use mc_legacy_formatting::{Color, Span, Styles};

#[test]
fn hub_mcs_gg() {
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

/// Tests that we support uppercase style codes
#[test]
fn purple_wtf() {
    let s = "§5§m                  §6>§7§l§6§l>§6§l[§5§l§oPurple §8§l§oPrison§6§l]§6§l<§6<§5§m                     \
                §R §7              (§4!§7) §e§lSERVER HAS §D§LRESET! §7(§4!§7)";
    assert_eq!(
        spans(s),
        vec![
            // The vanilla client renders whitespace with `Styles::STRIKETHROUGH`
            // as a solid line.
            Span::new_strikethrough_whitespace(
                "                  ",
                Color::DarkPurple,
                Styles::STRIKETHROUGH
            ),
            Span::new_styled(">", Color::Gold, Styles::empty()),
            Span::new_styled(">", Color::Gold, Styles::BOLD),
            Span::new_styled("[", Color::Gold, Styles::BOLD),
            Span::new_styled("Purple ", Color::DarkPurple, Styles::BOLD | Styles::ITALIC),
            Span::new_styled("Prison", Color::DarkGray, Styles::BOLD | Styles::ITALIC),
            Span::new_styled("]", Color::Gold, Styles::BOLD),
            Span::new_styled("<", Color::Gold, Styles::BOLD),
            Span::new_styled("<", Color::Gold, Styles::empty()),
            Span::new_strikethrough_whitespace(
                "                     ",
                Color::DarkPurple,
                Styles::STRIKETHROUGH
            ),
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

/// Tests whitespace strikethrough handling
#[test]
fn mc_mineheroes_org() {
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
fn play_mc_blaze_com() {
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

#[test]
fn play_lemoncloud_net() {
    let s = "§f§f §6§m---§e§m---§f§m---§f   §e§lLemon§f§lCloud§f §6[1.7-1.16] §f  §f§m---§e§m---§6§m---§f  \
                §f\n          §6§lSurvival 1.16 §e§l+ §c§lNether Reset!";
    assert_eq!(
        spans(s),
        vec![
            Span::new_plain(" "),
            Span::new_styled("---", Color::Gold, Styles::STRIKETHROUGH),
            Span::new_styled("---", Color::Yellow, Styles::STRIKETHROUGH),
            Span::new_styled("---", Color::White, Styles::STRIKETHROUGH),
            Span::new_plain("   "),
            Span::new_styled("Lemon", Color::Yellow, Styles::BOLD),
            Span::new_styled("Cloud", Color::White, Styles::BOLD),
            Span::new_plain(" "),
            Span::new_styled("[1.7-1.16] ", Color::Gold, Styles::empty()),
            Span::new_plain("  "),
            Span::new_styled("---", Color::White, Styles::STRIKETHROUGH),
            Span::new_styled("---", Color::Yellow, Styles::STRIKETHROUGH),
            Span::new_styled("---", Color::Gold, Styles::STRIKETHROUGH),
            Span::new_plain("  "),
            Span::new_plain("\n          "),
            Span::new_styled("Survival 1.16 ", Color::Gold, Styles::BOLD),
            Span::new_styled("+ ", Color::Yellow, Styles::BOLD),
            Span::new_styled("Nether Reset!", Color::Red, Styles::BOLD),
        ]
    );
}

#[test]
fn play_opblocks_com() {
    let s = "§c§lOPBlocks §8» §5§lM§b§li§5§ll§b§lk§5§ly§b§lW§5§la§b§ly §c§lPrison §8» §f§lOPEN NOW!     \
                §7Join our §3Discord §7here » §cdiscord.gg/opblocks";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled("OPBlocks ", Color::Red, Styles::BOLD),
            Span::new_styled("» ", Color::DarkGray, Styles::empty()),
            Span::new_styled("M", Color::DarkPurple, Styles::BOLD),
            Span::new_styled("i", Color::Aqua, Styles::BOLD),
            Span::new_styled("l", Color::DarkPurple, Styles::BOLD),
            Span::new_styled("k", Color::Aqua, Styles::BOLD),
            Span::new_styled("y", Color::DarkPurple, Styles::BOLD),
            Span::new_styled("W", Color::Aqua, Styles::BOLD),
            Span::new_styled("a", Color::DarkPurple, Styles::BOLD),
            Span::new_styled("y ", Color::Aqua, Styles::BOLD),
            Span::new_styled("Prison ", Color::Red, Styles::BOLD),
            Span::new_styled("» ", Color::DarkGray, Styles::empty()),
            Span::new_styled("OPEN NOW!     ", Color::White, Styles::BOLD),
            Span::new_styled("Join our ", Color::Gray, Styles::empty()),
            Span::new_styled("Discord ", Color::DarkAqua, Styles::empty()),
            Span::new_styled("here » ", Color::Gray, Styles::empty()),
            Span::new_styled("discord.gg/opblocks", Color::Red, Styles::empty()),
        ]
    );
}

#[test]
fn pixel_mc_complex_com() {
    let s =
        "§b§m-----------§8§l[- §f§lComplex §b§lGaming §8§l-]§b§m----------\n§fQuests/Clans §8| \
                §b#1 Pixelmon Network §8| §fCustom Plugins";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled("-----------", Color::Aqua, Styles::STRIKETHROUGH),
            Span::new_styled("[- ", Color::DarkGray, Styles::BOLD),
            Span::new_styled("Complex ", Color::White, Styles::BOLD),
            Span::new_styled("Gaming ", Color::Aqua, Styles::BOLD),
            Span::new_styled("-]", Color::DarkGray, Styles::BOLD),
            Span::new_styled("----------\n", Color::Aqua, Styles::STRIKETHROUGH),
            Span::new_plain("Quests/Clans "),
            Span::new_styled("| ", Color::DarkGray, Styles::empty()),
            Span::new_styled("#1 Pixelmon Network ", Color::Aqua, Styles::empty()),
            Span::new_styled("| ", Color::DarkGray, Styles::empty()),
            Span::new_plain("Custom Plugins"),
        ]
    );
}

#[test]
fn play_mcprison_com() {
    let s = "  §3§lMC§b§lPrison§7 | The Best Prison Experience... \n§b§lATLANTIC §a§lReleases§7 \
                on§a§l Saturday§7 at §a§l3PM EST!";
    assert_eq!(
        spans(s),
        vec![
            Span::new_plain("  "),
            Span::new_styled("MC", Color::DarkAqua, Styles::BOLD),
            Span::new_styled("Prison", Color::Aqua, Styles::BOLD),
            Span::new_styled(
                " | The Best Prison Experience... \n",
                Color::Gray,
                Styles::empty()
            ),
            Span::new_styled("ATLANTIC ", Color::Aqua, Styles::BOLD),
            Span::new_styled("Releases", Color::Green, Styles::BOLD),
            Span::new_styled(" on", Color::Gray, Styles::empty()),
            Span::new_styled(" Saturday", Color::Green, Styles::BOLD),
            Span::new_styled(" at ", Color::Gray, Styles::empty()),
            Span::new_styled("3PM EST!", Color::Green, Styles::BOLD),
        ]
    );
}

#[test]
fn play_pvpwars_net() {
    let s = "§6§l┍§e§l━ •§6§l•§f§l• §r[ §e§l§nP§6§l§nV§e§l§nP§f§n §6§l§nW§e§l§nA§6§l§nR§e§l§nS§r ] \
                §f§o§l—— §4»§c»§f» §c§l5§e§l0§6§l% §c§lS§e§lA§6§lL§c§lE §f«§c«§4« §f§l•§6§l•§e§l• \
                ━§6§l┑\n  §e§lNEWS: §c§lSB §4§l§nFIRE§r §r§l⁄ §3§l§nICE§r §b§lRESET§f§l! §f§o§m--§7 \
                §oJoin now!";
    assert_eq!(
        spans(s),
        vec![
            Span::new_styled("┍", Color::Gold, Styles::BOLD),
            Span::new_styled("━ •", Color::Yellow, Styles::BOLD),
            Span::new_styled("•", Color::Gold, Styles::BOLD),
            Span::new_styled("• ", Color::White, Styles::BOLD),
            Span::new_plain("[ "),
            Span::new_styled("P", Color::Yellow, Styles::BOLD | Styles::UNDERLINED),
            Span::new_styled("V", Color::Gold, Styles::BOLD | Styles::UNDERLINED),
            Span::new_styled("P", Color::Yellow, Styles::BOLD | Styles::UNDERLINED),
            Span::new_styled(" ", Color::White, Styles::UNDERLINED),
            Span::new_styled("W", Color::Gold, Styles::BOLD | Styles::UNDERLINED),
            Span::new_styled("A", Color::Yellow, Styles::BOLD | Styles::UNDERLINED),
            Span::new_styled("R", Color::Gold, Styles::BOLD | Styles::UNDERLINED),
            Span::new_styled("S", Color::Yellow, Styles::BOLD | Styles::UNDERLINED),
            Span::new_plain(" ] "),
            Span::new_styled("—— ", Color::White, Styles::BOLD | Styles::ITALIC),
            Span::new_styled("»", Color::DarkRed, Styles::empty()),
            Span::new_styled("»", Color::Red, Styles::empty()),
            Span::new_plain("» "),
            Span::new_styled("5", Color::Red, Styles::BOLD),
            Span::new_styled("0", Color::Yellow, Styles::BOLD),
            Span::new_styled("% ", Color::Gold, Styles::BOLD),
            Span::new_styled("S", Color::Red, Styles::BOLD),
            Span::new_styled("A", Color::Yellow, Styles::BOLD),
            Span::new_styled("L", Color::Gold, Styles::BOLD),
            Span::new_styled("E ", Color::Red, Styles::BOLD),
            Span::new_plain("«"),
            Span::new_styled("«", Color::Red, Styles::empty()),
            Span::new_styled("« ", Color::DarkRed, Styles::empty()),
            Span::new_styled("•", Color::White, Styles::BOLD),
            Span::new_styled("•", Color::Gold, Styles::BOLD),
            Span::new_styled("• ━", Color::Yellow, Styles::BOLD),
            Span::new_styled("┑\n  ", Color::Gold, Styles::BOLD),
            Span::new_styled("NEWS: ", Color::Yellow, Styles::BOLD),
            Span::new_styled("SB ", Color::Red, Styles::BOLD),
            Span::new_styled("FIRE", Color::DarkRed, Styles::BOLD | Styles::UNDERLINED),
            Span::new_plain(" "),
            Span::new_styled("⁄ ", Color::White, Styles::BOLD),
            Span::new_styled("ICE", Color::DarkAqua, Styles::BOLD | Styles::UNDERLINED),
            Span::new_plain(" "),
            Span::new_styled("RESET", Color::Aqua, Styles::BOLD),
            Span::new_styled("! ", Color::White, Styles::BOLD),
            Span::new_styled("--", Color::White, Styles::STRIKETHROUGH | Styles::ITALIC),
            Span::new_styled(" ", Color::Gray, Styles::empty()),
            Span::new_styled("Join now!", Color::Gray, Styles::ITALIC),
        ]
    );
}

#[test]
fn mccentral_org() {
    let s = " §c§lI§d§l§m-§c§l§m-§6§l§m-§e§l§m-§b§l§m-§8§l[§r §b§lMCCentral §d§lFactions Reset§r \
                §8§l]§b§l§m-§e§l§m-§6§l§m-§c§l§m-§d§l§m-§c§lI     §a§l23rd October 3pm MST§r \
                §f§l§m->§r §6§l$1125 In Prizes";
    assert_eq!(
        spans(s),
        vec![
            Span::new_plain(" "),
            Span::new_styled("I", Color::Red, Styles::BOLD),
            Span::new_styled(
                "-",
                Color::LightPurple,
                Styles::BOLD | Styles::STRIKETHROUGH
            ),
            Span::new_styled("-", Color::Red, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("-", Color::Gold, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("-", Color::Yellow, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("-", Color::Aqua, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("[", Color::DarkGray, Styles::BOLD),
            Span::new_plain(" "),
            Span::new_styled("MCCentral ", Color::Aqua, Styles::BOLD),
            Span::new_styled("Factions Reset", Color::LightPurple, Styles::BOLD),
            Span::new_plain(" "),
            Span::new_styled("]", Color::DarkGray, Styles::BOLD),
            Span::new_styled("-", Color::Aqua, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("-", Color::Yellow, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("-", Color::Gold, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled("-", Color::Red, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_styled(
                "-",
                Color::LightPurple,
                Styles::BOLD | Styles::STRIKETHROUGH
            ),
            Span::new_styled("I     ", Color::Red, Styles::BOLD),
            Span::new_styled("23rd October 3pm MST", Color::Green, Styles::BOLD),
            Span::new_plain(" "),
            Span::new_styled("->", Color::White, Styles::BOLD | Styles::STRIKETHROUGH),
            Span::new_plain(" "),
            Span::new_styled("$1125 In Prizes", Color::Gold, Styles::BOLD),
        ]
    );
}
