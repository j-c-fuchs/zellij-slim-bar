use zellij_tile::prelude::*;

#[derive(Default)]
struct SlimBar {}

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
        false
    }

    fn render(&mut self, rows: usize, cols: usize) {
        return;
    }
}
