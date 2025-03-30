use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};
use fltk_observe::{Runner, WidgetObserver};

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

    fn increment(&mut self, _b: &Button) {
        self.value += 1;
    }

    fn just_decrement(&mut self) {
        self.value -= 1;
    }

    fn decrement(&mut self, _b: &impl WidgetExt) {
        self.just_decrement();
    }

    fn update_label(&self, f: &mut Frame) {
        f.set_label(&self.value().to_string());
    }
}

fn main() {
    let a = app::App::default().with_state(Counter::new);

    let mut window = Window::default().with_size(320, 240).with_label("Add data");
    let col = Flex::default_fill().column();
    let mut inc = Button::default().with_label("+");
    inc.set_action(Counter::increment);
    let mut f = Frame::default();
    f.set_view(Counter::update_label);
    let mut dec = Button::default().with_label("-");
    dec.set_action(|s: &mut Counter, b| s.decrement(b));
    col.end();
    window.end();
    window.show();

    fltk_observe::use_state_mut(|c: &mut Counter| c.value += 1);
    fltk_observe::use_state_mut(Counter::just_decrement);

    a.run().unwrap();
}
