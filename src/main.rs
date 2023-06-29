use std::cmp::{max, min};

use zellij_tile::prelude::*;

#[derive(Default)]
struct SlimBar {
    tabs: Vec<TabInfo>,
    active_tab_idx: usize,
    mode_info: ModeInfo,
    mouse_click_pos: usize,
    should_change_tab: bool,
}

register_plugin!(SlimBar);

impl ZellijPlugin for SlimBar {
    fn load(&mut self) {
        set_selectable(false);
        subscribe(&[
            EventType::TabUpdate,
            EventType::ModeUpdate,
            EventType::Mouse,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        let mut should_render = false;
        match event {
            Event::ModeUpdate(mode_info) => {
                if self.mode_info != mode_info {
                    should_render = true;
                    self.mode_info = mode_info
                }
            },
            Event::TabUpdate(tabs) => {
                if let Some(active_tab_index) = tabs.iter().position(|t| t.active) {
                    // tabs are indexed starting from 1 so we need to add 1
                    let active_tab_idx = active_tab_index + 1;
                    if self.active_tab_idx != active_tab_idx || self.tabs != tabs {
                        should_render = true;
                    }
                    self.active_tab_idx = active_tab_idx;
                    self.tabs = tabs;
                } else {
                    eprintln!("Could not find active tab.");
                }
            },
            Event::Mouse(me) => match me {
                Mouse::LeftClick(_, col) => {
                    if self.mouse_click_pos != col {
                        should_render = true;
                        self.should_change_tab = true;
                        self.mouse_click_pos = col;
                    }
                },
                Mouse::ScrollUp(_) => {
                    should_render = true;
                    switch_tab_to(min(self.active_tab_idx + 1, self.tabs.len()) as u32);
                },
                Mouse::ScrollDown(_) => {
                    should_render = true;
                    switch_tab_to(max(self.active_tab_idx.saturating_sub(1), 1) as u32);
                },
                _ => {},
            },
            _ => {
                eprintln!("Got unrecognized event: {:?}", event);
            },
        };
        should_render
    }

    fn render(&mut self, rows: usize, cols: usize) {
        return;
    }
}
