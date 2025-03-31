use fltk::{enums::Shortcut, menu::MenuFlag, prelude::*, *};
use fltk_observe::*;

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }

    fn value(&self) -> i32 {
        self.value
    }

    fn increment(&mut self, _m: &menu::MenuBar) {
        self.value += 1;
    }

    fn decrement(&mut self, _m: &impl MenuExt) {
        self.value -= 1;
    }

    fn update_frame(&self, o: &mut frame::Frame) {
        o.set_label(&self.value().to_string());
    }
}

fn main() {
    let a = app::App::default().use_state(Counter::new).unwrap();
    let mut w = window::Window::default().with_size(400, 300);
    let mut col = group::Flex::default_fill().column();
    let mut menu = menu::MenuBar::default();
    col.fixed(&menu, 30);
    menu.add_action(
        "File/Increment",
        Shortcut::None,
        MenuFlag::Normal,
        Counter::increment,
    );
    menu.add_action(
        "File/Decrement",
        Shortcut::None,
        MenuFlag::Normal,
        Counter::decrement,
    );
    let mut f = frame::Frame::default();
    f.set_view(Counter::update_frame);
    col.end();
    w.end();
    w.show();

    a.run().unwrap();
}
