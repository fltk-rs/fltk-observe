# fltk-observe

An observer pattern for fltk-rs.

## Usage
```toml
[dependencies]
fltk = "1.5"
fltk-observe = "0.1.0"
```

## Example
```rust,no_run
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
```

Example with an async runtime:
```rust,ignore
use fltk::{prelude::*, *};
use fltk_observe::{Runner, WidgetObserver};
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
struct State {
    value: Arc<Mutex<String>>,
}

impl State {
    pub fn fetch(&mut self, _b: &button::Button) {
        let value = self.value.clone();
        tokio::spawn(async move {
            *value.lock().unwrap() = reqwest::get("https://www.example.com")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            fltk_observe::notify();
        });
    }

    pub fn update_hv(&self, hv: &mut misc::HelpView) {
        let value = self.value.clone();
        hv.set_value(&value.lock().unwrap());
    }
}

#[tokio::main]
async fn main() {
    let a = app::App::default().with_state(State::default);
    let mut w = window::Window::default().with_size(400, 300);
    let mut col = group::Flex::default_fill().column();
    let mut hv = misc::HelpView::default();
    hv.set_view(State::update_hv);
    let mut btn = button::Button::default().with_label("Fetch");
    btn.set_action(State::fetch);
    col.fixed(&btn, 30);
    col.end();
    w.end();
    w.show();
    a.run().unwrap();
}
```