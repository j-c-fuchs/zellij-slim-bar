use unicode_width::UnicodeWidthStr;

use crate::LinePart;
use zellij_tile::prelude::*;
use zellij_tile_utils::style;

fn get_current_title_len(current_title: &[LinePart]) -> usize {
    current_title.iter().map(|p| p.len).sum()
}

// move elements from before_active and after_active into tabs_to_render while they fit in cols
// adds collapsed_tabs to the left and right if there's left over tabs that don't fit
fn populate_tabs_in_tab_line(
    tabs_before_active: &mut Vec<LinePart>,
    tabs_after_active: &mut Vec<LinePart>,
    tabs_to_render: &mut Vec<LinePart>,
    cols: usize,
    palette: Palette,
) {
    let mut middle_size = get_current_title_len(tabs_to_render);

    let mut total_left = 0;
    let mut total_right = 0;
    loop {
        let left_count = tabs_before_active.len();
        let right_count = tabs_after_active.len();

        // left_more_tab_index is the tab to the left of the leftmost visible tab
        let left_more_tab_index = left_count.saturating_sub(1);
        let collapsed_left = left_more_message(
            left_count,
            palette,
            left_more_tab_index,
        );
        // right_more_tab_index is the tab to the right of the rightmost visible tab
        let right_more_tab_index = left_count + tabs_to_render.len();
        let collapsed_right = right_more_message(
            right_count,
            palette,
            right_more_tab_index,
        );

        let total_size = collapsed_left.len + middle_size + collapsed_right.len;

        if total_size > cols {
            // break and dont add collapsed tabs to tabs_to_render, they will not fit
            break;
        }

        let left = if let Some(tab) = tabs_before_active.last() {
            tab.len
        } else {
            usize::MAX
        };

        let right = if let Some(tab) = tabs_after_active.first() {
            tab.len
        } else {
            usize::MAX
        };

        // total size is shortened if the next tab to be added is the last one, as that will remove the collapsed tab
        let size_by_adding_left =
            left.saturating_add(total_size)
                .saturating_sub(if left_count == 1 {
                    collapsed_left.len
                } else {
                    0
                });
        let size_by_adding_right =
            right
                .saturating_add(total_size)
                .saturating_sub(if right_count == 1 {
                    collapsed_right.len
                } else {
                    0
                });

        let left_fits = size_by_adding_left <= cols;
        let right_fits = size_by_adding_right <= cols;
        // active tab is kept in the middle by adding to the side that
        // has less width, or if the tab on the other side doesn't fit
        if (total_left <= total_right || !right_fits) && left_fits {
            // add left tab
            let tab = tabs_before_active.pop().unwrap();
            middle_size += tab.len;
            total_left += tab.len;
            tabs_to_render.insert(0, tab);
        } else if right_fits {
            // add right tab
            let tab = tabs_after_active.remove(0);
            middle_size += tab.len;
            total_right += tab.len;
            tabs_to_render.push(tab);
        } else {
            // there's either no space to add more tabs or no more tabs to add, so we're done
            tabs_to_render.insert(0, collapsed_left);
            tabs_to_render.push(collapsed_right);
            break;
        }
    }
}

fn left_more_message(
    tab_count_to_the_left: usize,
    palette: Palette,
    tab_index: usize,
) -> LinePart {
    if tab_count_to_the_left == 0 {
        return LinePart::default();
    }
    let more_text = if tab_count_to_the_left < 10 {
        format!(" << {} ", tab_count_to_the_left)
    } else {
        " <<< ".to_string()
    };
    let more_text_len = more_text.width();
    let (text_color, _sep_color) = match palette.theme_hue {
        ThemeHue::Dark => (palette.white, palette.black),
        ThemeHue::Light => (palette.black, palette.white),
    };
    let more_styled_text = style!(text_color, palette.orange)
        .bold()
        .paint(more_text)
        .to_string();
    LinePart {
        part: more_styled_text,
        len: more_text_len,
        tab_index: Some(tab_index),
    }
}

fn right_more_message(
    tab_count_to_the_right: usize,
    palette: Palette,
    tab_index: usize,
) -> LinePart {
    if tab_count_to_the_right == 0 {
        return LinePart::default();
    };
    let more_text = if tab_count_to_the_right < 10 {
        format!(" {} >> ", tab_count_to_the_right)
    } else {
        " >>> ".to_string()
    };
    let more_text_len = more_text.width();
    let (text_color, _sep_color) = match palette.theme_hue {
        ThemeHue::Dark => (palette.white, palette.black),
        ThemeHue::Light => (palette.black, palette.white),
    };
    let more_styled_text = style!(text_color, palette.orange)
        .bold()
        .paint(more_text)
        .to_string();
    LinePart {
        part: more_styled_text,
        len: more_text_len,
        tab_index: Some(tab_index),
    }
}

fn session_part(session_name: Option<&str>, palette: Palette) -> LinePart {
    let bg_color = match palette.theme_hue {
        ThemeHue::Dark => palette.black,
        ThemeHue::Light => palette.white,
    };

    let name = match session_name {
        Some(n) => n,
        None => "<unnamed>",
    };
    let name_part = format!(" {} ", name);
    let name_part_len = name_part.width();
    let text_color = match palette.theme_hue {
        ThemeHue::Dark => palette.white,
        ThemeHue::Light => palette.black,
    };
    let name_part_styled_text = style!(text_color, bg_color).bold().paint(name_part);
    LinePart {
        part: name_part_styled_text.to_string(),
        len: name_part_len,
        tab_index: None,
    }
}

fn mode_part(mode: InputMode, palette: Palette) -> LinePart {
    let bg_color = match palette.theme_hue {
        ThemeHue::Dark => palette.black,
        ThemeHue::Light => palette.white,
    };

    let locked_mode_color = palette.magenta;
    let normal_mode_color = palette.green;
    let other_modes_color = palette.orange;

    let mode_part = format!("{:?}", mode).to_uppercase();
    let mode_part_padded = format!("{:^8}", mode_part);
    let mode_part_len = mode_part_padded.width();
    let mode_part_styled_text = if mode == InputMode::Locked {
        style!(locked_mode_color, bg_color)
            .bold()
            .paint(mode_part_padded)
    } else if mode == InputMode::Normal {
        style!(normal_mode_color, bg_color)
            .bold()
            .paint(mode_part_padded)
    } else {
        style!(other_modes_color, bg_color)
            .bold()
            .paint(mode_part_padded)
    };
    LinePart {
        part: format!("{}", mode_part_styled_text),
        len: mode_part_len,
        tab_index: None,
    }
}

pub fn bar_line(
    session_name: Option<&str>,
    mut all_tabs: Vec<LinePart>,
    active_tab_index: usize,
    cols: usize,
    palette: Palette,
    mode: InputMode,
    active_swap_layout_name: &Option<String>,
    is_swap_layout_dirty: bool,
) -> Vec<LinePart> {
    let mut tabs_after_active = all_tabs.split_off(active_tab_index);
    let mut tabs_before_active = all_tabs;
    let active_tab = if !tabs_after_active.is_empty() {
        tabs_after_active.remove(0)
    } else {
        tabs_before_active.pop().unwrap()
    };

    let mut left_parts = vec![mode_part(mode, palette)];
    let left_len = get_current_title_len(&left_parts);
    if left_len + active_tab.len > cols {
        return left_parts;
    }

    let mut right_parts: Vec<LinePart> = [
        swap_layout_status(
            active_swap_layout_name,
            is_swap_layout_dirty,
            mode,
            &palette,
        ),
        Some(session_part(session_name, palette)),
    ].into_iter().filter_map(|x| x).collect();

    while !right_parts.is_empty() && left_len + active_tab.len + get_current_title_len(&right_parts) > cols {
        right_parts.remove(0);
    }

    let mut tabs_to_render = vec![active_tab];

    populate_tabs_in_tab_line(
        &mut tabs_before_active,
        &mut tabs_after_active,
        &mut tabs_to_render,
        cols.saturating_sub(left_len + get_current_title_len(&right_parts)),
        palette,
    );
    left_parts.append(&mut tabs_to_render);
    left_parts.append(&mut right_parts);

    left_parts
}

fn swap_layout_status(
    swap_layout_name: &Option<String>,
    is_swap_layout_damaged: bool,
    input_mode: InputMode,
    palette: &Palette,
) -> Option<LinePart> {
    match swap_layout_name {
        Some(swap_layout_name) => {
            let mut swap_layout_name = format!(" {} ", swap_layout_name);
            swap_layout_name.make_ascii_uppercase();
            let swap_layout_name_len = swap_layout_name.len() + 3;

            let swap_layout_name =
                if input_mode == InputMode::Locked {
                    style!(palette.black, palette.fg)
                        .italic()
                        .paint(&swap_layout_name)
                } else if is_swap_layout_damaged {
                    style!(palette.black, palette.fg)
                        .bold()
                        .paint(&swap_layout_name)
                } else {
                    style!(palette.black, palette.green)
                        .bold()
                        .paint(&swap_layout_name)
                };
            Some(LinePart {
                part: swap_layout_name.to_string(),
                len: swap_layout_name_len,
                tab_index: None,
            })
        },
        None => None,
    }
}
