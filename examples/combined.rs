use fltk::{app, button::Button, prelude::*, window::Window};
use fltk_observe::sync::{Runner, WidgetObserver};

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

    fn update_label(&self, b: &mut Button) {
        b.set_label(&self.value().to_string());
    }
}

fn main() {
    let a = app::App::default().with_state(Counter::new);

    let mut window = Window::default().with_size(200, 200).with_label("Add data");
    let mut inc = Button::default_fill();
    inc.set_action(Counter::increment);
    inc.set_view(Counter::update_label);
    window.end();
    window.show();

    a.run().unwrap();
}
