use crate::LinePart;
use ansi_term::{ANSIString, ANSIStrings};
use unicode_width::UnicodeWidthStr;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

fn cursors(focused_clients: &[ClientId], palette: Palette) -> (Vec<ANSIString>, usize) {
    // cursor section, text length
    let mut len = 0;
    let mut cursors = vec![];
    for client_id in focused_clients.iter() {
        if let Some(color) = client_id_to_colors(*client_id, palette) {
            cursors.push(style!(color.1, color.0).paint(" "));
            len += 1;
        }
    }
    (cursors, len)
}

pub fn render_tab(
    text: String,
    tab: &TabInfo,
    is_alternate_tab: bool,
    palette: Palette,
    separator: &str,
) -> LinePart {
    let focused_clients = tab.other_focused_clients.as_slice();
    let separator_width = separator.width();
    let alternate_tab_color = match palette.theme_hue {
        // TODO: only do this if we don't have the arrow capabilities
        ThemeHue::Dark => palette.white,
        ThemeHue::Light => palette.black,
    };
    let background_color = if tab.active {
        palette.green
    } else if is_alternate_tab {
        alternate_tab_color
    } else {
        palette.fg
    };
    let foreground_color = match palette.theme_hue {
        ThemeHue::Dark => palette.black,
        ThemeHue::Light => palette.white,
    };
    let left_separator = style!(foreground_color, background_color).paint(separator);
    let mut tab_text_len = text.width() + (separator_width * 2) + 2; // + 2 for padding

    let tab_styled_text = style!(foreground_color, background_color)
        .bold()
        .paint(format!(" {} ", text));

    let right_separator = style!(background_color, foreground_color).paint(separator);
    let tab_styled_text = if !focused_clients.is_empty() {
        let (cursor_section, extra_length) = cursors(focused_clients, palette);
        tab_text_len += extra_length;
        let mut s = String::new();
        let cursor_beginning = style!(foreground_color, background_color)
            .bold()
            .paint("[")
            .to_string();
        let cursor_section = ANSIStrings(&cursor_section).to_string();
        let cursor_end = style!(foreground_color, background_color)
            .bold()
            .paint("]")
            .to_string();
        s.push_str(&left_separator.to_string());
        s.push_str(&tab_styled_text.to_string());
        s.push_str(&cursor_beginning);
        s.push_str(&cursor_section);
        s.push_str(&cursor_end);
        s.push_str(&right_separator.to_string());
        s
    } else {
        ANSIStrings(&[left_separator, tab_styled_text, right_separator]).to_string()
    };

    LinePart {
        part: tab_styled_text,
        len: tab_text_len,
        tab_index: Some(tab.position),
    }
}

pub fn tab_style(
    mut tabname: String,
    tab: &TabInfo,
    mut is_alternate_tab: bool,
    palette: Palette,
    capabilities: PluginCapabilities,
) -> LinePart {
    let separator = "";
    if tab.is_sync_panes_active {
        tabname.push_str(" (Sync)");
    }

    render_tab(tabname, tab, is_alternate_tab, palette, separator)
}
