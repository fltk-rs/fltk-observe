use fltk::{prelude::*, *};
use fltk_observe::sync::*;

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

    fn change_counter(&mut self, m: &menu::MenuBar) {
        if let Ok(path) = m.item_pathname(None) {
            match path.as_str() {
                "File/Increment" => self.value += 1,
                "File/Decrement" => self.value -= 1,
                _ => (),
            }
        }
    }

    fn update_frame(&self, o: &mut frame::Frame) {
        o.set_label(&self.value().to_string());
    }
}

fn main() {
    let a = app::App::default().with_state(Counter::new);
    let mut w = window::Window::default().with_size(400, 300);
    let mut col = group::Flex::default_fill().column();
    let mut menu = menu::MenuBar::default();
    col.fixed(&menu, 30);
    menu.set_action(Counter::change_counter);
    menu.add_choice("File/Increment|File/Decrement");
    let mut f = frame::Frame::default();
    f.set_view(Counter::update_frame);
    col.end();
    w.end();
    w.show();

    a.run().unwrap();
}
