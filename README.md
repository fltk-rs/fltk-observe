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
    let a = app::App::default().use_state(Counter::new).unwrap();

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

    fltk_observe::with_state_mut(|c: &mut Counter| c.value += 1);
    fltk_observe::with_state_mut(Counter::just_decrement);

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
    let a = app::App::default().use_state(State::default).unwrap();
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

fltk-observe also enables using an mvvm or mvc pattern (check the examples) with fltk-rs:
```rust,no_run
use fltk::{app, button::Button, frame::Frame, group::Flex, prelude::*, window::Window};

struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }
    fn increment(&mut self) {
        self.value += 1;
    }
    fn decrement(&mut self) {
        self.value -= 1;
    }
    fn get_value(&self) -> i32 {
        self.value
    }
}

struct CounterViewModel {
    model: Counter,
}

impl CounterViewModel {
    fn new() -> Self {
        Self {
            model: Counter::new(),
        }
    }

    fn increment(&mut self, _btn: &Button) {
        self.model.increment();
    }

    fn decrement(&mut self, _btn: &Button) {
        self.model.decrement();
    }

    fn update_display(&self, frame: &mut Frame) {
        frame.set_label(&self.model.get_value().to_string());
    }
}

struct CounterView {
    inc_btn: Button,
    dec_btn: Button,
    display: Frame,
}

impl CounterView {
    fn new() -> Self {
        let mut window = Window::default()
            .with_size(300, 160)
            .with_label("MVVM (fltk-observe)");
        let flex = Flex::default_fill().column();
        let inc_btn = Button::default().with_label("Increment");
        let display = Frame::default();
        let dec_btn = Button::default().with_label("Decrement");
        flex.end();
        window.end();
        window.show();
        Self {
            inc_btn,
            dec_btn,
            display,
        }
    }
}

struct CounterApp {
    app: app::App,
}

impl CounterApp {
    fn new() -> Self {
        use fltk_observe::{Runner, WidgetObserver};
        let app = app::App::default()
            .use_state(CounterViewModel::new)
            .unwrap();

        let mut view = CounterView::new();
        view.inc_btn.set_action(CounterViewModel::increment);
        view.dec_btn.set_action(CounterViewModel::decrement);
        view.display.set_view(CounterViewModel::update_display);

        Self { app }
    }

    fn run(&self) {
        self.app.run().unwrap();
    }
}

fn main() {
    let app = CounterApp::new();
    app.run();
}
```
