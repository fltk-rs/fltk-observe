use fltk::{prelude::*, *};
use fltk_observe::asynch::{Runner, WidgetObserver};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default, Clone)]
struct State {
    value: Arc<Mutex<String>>,
}

impl State {
    pub async fn fetch(self: Arc<Self>, _b: Arc<button::Button>) {
        let value = self.value.clone();
        *value.lock().await = reqwest::get("https://www.example.com")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
    }

    pub async fn update_hv(self: Arc<Self>, mut hv: misc::HelpView) {
        let value = self.value.clone();
        hv.set_value(&value.lock().await);
    }
}

#[tokio::main]
async fn main() {
    let a = app::App::default().with_state(State::default).await;
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
