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
