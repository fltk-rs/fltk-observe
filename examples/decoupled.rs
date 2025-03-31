use fltk::{app, button::Button, prelude::*, window::Window};
use fltk_observe::{Runner, WidgetObserver};
use std::sync::{Arc, Mutex};

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
}

fn increment(c: &mut Arc<Mutex<Counter>>, _b: &Button) {
    c.lock().unwrap().value += 1;
}

fn update_label(c: &Arc<Mutex<Counter>>, b: &mut Button) {
    b.set_label(&c.lock().unwrap().value().to_string());
}

fn main() {
    let a = app::App::default().use_state(|| Arc::new(Mutex::new(Counter::new()))).unwrap();

    let mut window = Window::default().with_size(200, 200).with_label("Add data");
    let mut inc = Button::default_fill();
    inc.set_action(increment);
    inc.set_view(update_label);
    window.end();
    window.show();

    a.run().unwrap();
}
