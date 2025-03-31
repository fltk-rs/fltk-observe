// Try accessing localhost:3000 from the browser

use axum::{Router, extract::Path, response::Html, routing::get};
use fltk::{prelude::*, *};
use fltk_observe::{Runner, WidgetObserver};
use std::sync::{Arc, Mutex};

struct State {
    value: Arc<Mutex<String>>,
}

impl State {
    pub fn new() -> Self {
        let value = Arc::new(Mutex::new(String::new()));
        let s_cl = value.clone();
        tokio::spawn(async move {
            let app = Router::new()
                .route(
                    "/",
                    get({
                        let s_cl = s_cl.clone();
                        move || {
                            *s_cl.lock().unwrap() = "Got request to /".to_string();
                            fltk_observe::notify();
                            async { Html("Hello from Axum + fltk-observe!\n") }
                        }
                    }),
                )
                .route(
                    "/{p}",
                    get({
                        let s_cl = s_cl.clone();
                        move |path: Path<String>| {
                            *s_cl.lock().unwrap() = format!("Got request to /{}", path.0);
                            fltk_observe::notify();
                            async { Html("Hello from Axum + fltk-observe!\n") }
                        }
                    }),
                );
            axum::serve(
                tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap(),
                app,
            )
            .await
            .unwrap();
        });
        Self { value }
    }

    pub fn update_label(&self, f: &mut frame::Frame) {
        f.set_label(&self.value.lock().unwrap());
    }
}

#[tokio::main]
async fn main() {
    let a = app::App::default().use_state(State::new).unwrap();
    let mut w = window::Window::default().with_size(400, 300);
    let mut f = frame::Frame::default_fill();
    f.set_view(State::update_label);
    w.end();
    w.show();
    a.run().unwrap();
}
