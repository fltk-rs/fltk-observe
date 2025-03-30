#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::{
    app,
    enums::{Event, Shortcut},
    menu::MenuFlag,
    prelude::*,
};
use std::{any::Any, sync::Mutex};

pub const STATE_CHANGED: Event = Event::from_i32(100);

static STATE: Mutex<Option<Box<dyn Any + Send + Sync>>> = Mutex::new(None);

pub trait WidgetObserver<T, W> {
    fn set_action<Listen: Clone + 'static + Fn(&mut T, &Self)>(&mut self, l: Listen);
    fn set_view<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update);
}

pub trait MenuObserver<T, W> {
    fn add_action<Listen: Clone + 'static + Fn(&mut T, &Self)>(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flags: MenuFlag,
        l: Listen,
    );
}

impl<T: Send + Sync + 'static, W: WidgetExt + WidgetBase + 'static + Clone> WidgetObserver<T, W>
    for W
{
    fn set_action<Listen: Clone + 'static + Fn(&mut T, &Self)>(&mut self, l: Listen) {
        self.set_callback(move |w| {
            l(
                STATE
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .downcast_mut()
                    .unwrap(),
                w,
            );
            app::handle_main(STATE_CHANGED).ok();
        });
    }

    fn set_view<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update) {
        let w = self.clone();
        let func = move || {
            let mut w = w.clone();
            u(
                STATE
                    .lock()
                    .unwrap()
                    .as_ref()
                    .unwrap()
                    .downcast_ref()
                    .unwrap(),
                &mut w,
            );
        };
        func();
        self.handle(move |_w, ev| {
            if ev == STATE_CHANGED {
                func();
            }
            false
        });
    }
}

impl<T: Send + Sync + 'static, W: MenuExt + 'static + Clone> MenuObserver<T, W> for W {
    fn add_action<Listen: Clone + 'static + Fn(&mut T, &Self)>(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flags: MenuFlag,
        l: Listen,
    ) {
        self.add(label, shortcut, flags, move |w| {
            l(
                STATE
                    .lock()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .downcast_mut()
                    .unwrap(),
                w,
            );
            app::handle_main(STATE_CHANGED).ok();
        });
    }
}

pub trait Runner<State: 'static + Send + Sync> {
    fn with_state<F: 'static + FnMut() -> State>(self, init: F) -> Self
    where
        Self: Sized;
}

impl<State: 'static + Send + Sync> Runner<State> for app::App {
    fn with_state<F: 'static + FnMut() -> State>(self, mut init: F) -> Self
    where
        Self: Sized,
    {
        *STATE.lock().unwrap() = Some(Box::new((init)()));
        self
    }
}

pub fn use_state_mut<State: 'static, F: FnMut(&mut State) + Clone>(mut f: F) {
    f(STATE
        .lock()
        .unwrap()
        .as_mut()
        .unwrap()
        .downcast_mut()
        .unwrap());
    app::handle_main(STATE_CHANGED).ok();
}

pub fn use_state<State: 'static, F: FnMut(&State) + Clone>(mut f: F) {
    f(STATE
        .lock()
        .unwrap()
        .as_ref()
        .unwrap()
        .downcast_ref()
        .unwrap());
}

pub fn notify() {
    app::handle_main(STATE_CHANGED).ok();
    app::awake();
}
