#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::{
    app,
    enums::{Event, Shortcut},
    menu::MenuFlag,
    prelude::*,
};
use std::{
    any::Any,
    sync::{Mutex, OnceLock},
};

pub const STATE_CHANGED: Event = Event::from_i32(100);

static STATE: OnceLock<Mutex<Box<dyn Any + Send + Sync>>> = OnceLock::new();

macro_rules! state_ref {
    () => {
        STATE
            .get()
            .expect("Global state not initialized.")
            .lock()
            .expect("Failed to lock global state.")
            .downcast_ref()
            .expect("State type mismatch (did you init a different type?)")
    };
}

macro_rules! state_mut {
    () => {
        STATE
            .get()
            .expect("Global state not initialized.")
            .lock()
            .expect("Failed to lock global state.")
            .downcast_mut()
            .expect("State type mismatch (did you init a different type?)")
    };
}

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
            l(state_mut!(), w);
            app::handle_main(STATE_CHANGED).ok();
        });
    }

    fn set_view<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update) {
        let w = self.clone();
        let func = move || {
            let mut w = w.clone();
            u(state_ref!(), &mut w);
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
            l(state_mut!(), w);
            app::handle_main(STATE_CHANGED).ok();
        });
    }
}

pub trait Runner<State: 'static + Send + Sync> {
    fn use_state<F: 'static + FnOnce() -> State>(self, init: F) -> Option<Self>
    where
        Self: Sized;
}

impl<State: 'static + Send + Sync> Runner<State> for app::App {
    fn use_state<F: 'static + FnOnce() -> State>(self, init: F) -> Option<Self>
    where
        Self: Sized,
    {
        STATE.set(Mutex::new(Box::new((init)()))).ok()?;
        Some(self)
    }
}

pub fn with_state_mut<State: 'static, F: FnOnce(&mut State) + Clone>(f: F) {
    f(state_mut!());
    app::handle_main(STATE_CHANGED).ok();
}

pub fn with_state<State: 'static, F: FnOnce(&State) + Clone>(f: F) {
    f(state_ref!());
}

pub fn notify() {
    app::handle_main(STATE_CHANGED).ok();
    app::awake();
}
