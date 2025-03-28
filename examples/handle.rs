use fltk::{
    app,
    button::Button,
    enums::{Color, Event, FrameType},
    frame::Frame,
    group::Flex,
    prelude::*,
    window::Window,
};
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

    fn decrement(&mut self, _b: &impl WidgetExt) {
        self.value -= 1;
    }

    fn update_output(&self, f: &mut Frame) {
        f.set_label(&self.value().to_string());
    }
}

fn main() {
    let a = app::App::default().with_state(Counter::new);

    let mut window = Window::default().with_size(320, 240).with_label("Add data");
    let col = Flex::default_fill().column();
    let mut inc = Button::default().with_label("+");
    inc.set_action(Counter::increment);
    let mut f = Frame::default().with_label("0");
    f.set_frame(FrameType::FlatBox);
    f.handle(|f, ev| match ev {
        Event::Enter => {
            f.set_color(Color::Red);
            f.redraw();
            true
        }
        Event::Leave => {
            f.set_color(Color::Background);
            f.redraw();
            true
        }
        fltk_observe::STATE_CHANGED => {
            fltk_observe::use_state({
                let mut f = f.clone();
                move |s: &Counter| s.update_output(&mut f)
            });
            false
        }
        _ => false,
    });
    let mut dec = Button::default().with_label("-");
    dec.set_action(|s: &mut Counter, b| s.decrement(b));
    col.end();
    window.end();
    window.show();

    a.run().unwrap();
}
