use fltk::{
    app,
    button::Button,
    frame::Frame,
    group::Flex,
    prelude::*,
    window::Window,
};
use fltk_observe::{Runner, WidgetObserver};

struct CounterModel {
    value: i32,
}

impl CounterModel {
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

struct CounterView {
    pub window: Window,
    pub inc_btn: Button,
    pub dec_btn: Button,
    pub display: Frame,
}

impl CounterView {
    fn new() -> Self {
        let mut window = Window::default().with_size(300, 160).with_label("MVC (fltk-observe)");
        let mut flex = Flex::default_fill().column();
        let inc_btn = Button::default().with_label("Increment");
        let display = Frame::default();
        let dec_btn = Button::default().with_label("Decrement");
        flex.end();
        window.end();
        window.show();
        Self { window, inc_btn, dec_btn, display }
    }
}

struct CounterController {
    app: app::App,
    view: CounterView,
}

impl CounterController {
    fn new() -> Self {
        let app = app::App::default()
            .use_state(CounterModel::new)
            .unwrap();
        let mut view = CounterView::new();
        view.inc_btn.set_action(Self::handle_increment);
        view.dec_btn.set_action(Self::handle_decrement);
        view.display.set_view(Self::update_label);
        Self { app, view }
    }

    fn handle_increment(model: &mut CounterModel, _btn: &Button) {
        model.increment();
    }

    fn handle_decrement(model: &mut CounterModel, _btn: &Button) {
        model.decrement();
    }

    fn update_label(model: &CounterModel, frame: &mut Frame) {
        frame.set_label(&model.get_value().to_string());
    }

    fn run(&self) {
        self.app.run().unwrap();
    }
}

fn main() {
    let controller = CounterController::new();
    controller.run();
}
