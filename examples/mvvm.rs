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
