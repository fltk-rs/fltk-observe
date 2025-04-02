#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::{
    app,
    enums::{Event, Shortcut},
    menu::MenuFlag,
    prelude::*,
    window::Window,
};
use std::{
    any::Any,
    sync::{Mutex, OnceLock},
};

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

/// The event used to trigger UI updates when state changes.
pub const STATE_CHANGED: Event = Event::from_i32(100);

/// A trait to bind actions and views to widgets in response to shared state.
pub trait WidgetObserver<T, W> {
    /// Sets the action to be executed when the widget is interacted with.
    /// The function receives mutable access to the shared state and a reference to the widget.
    fn set_action<Listen: Clone + 'static + Fn(&mut T, &Self)>(&mut self, l: Listen);
    /// Binds a view function that updates the widget whenever the state changes.
    /// This is automatically triggered on `STATE_CHANGED`.
    fn set_view<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update);
}

/// A trait to bind state-aware actions to FLTK menu items.
pub trait MenuObserver<T, W> {
    /// Adds a menu item and attaches a state-aware action to it.
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
            let win = unsafe { Window::from_widget_ptr(w.window().unwrap().as_widget_ptr()) };
            l(state_mut!(), w);
            app::handle(STATE_CHANGED, &win.clone()).ok();
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
            let win = unsafe { Window::from_widget_ptr(w.window().unwrap().as_widget_ptr()) };
            l(state_mut!(), w);
            app::handle(STATE_CHANGED, &win).ok();
        });
    }
}

/// Provides the ability to initialize global application state for observation.
pub trait Runner<State: 'static + Send + Sync> {
    /// Initializes the global state using the given closure.
    /// Should be called early in `main()` before accessing state.
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

/// Mutably accesses the global state and applies the given closure.
/// Triggers a UI update by sending `STATE_CHANGED` to the main window.
pub fn with_state_mut<State: 'static, F: FnOnce(&mut State) + Clone>(f: F) {
    f(state_mut!());
    app::handle_main(STATE_CHANGED).ok();
}

/// Mutably accesses the global state and applies the given closure.
/// Also emits `STATE_CHANGED` to the given window.
pub fn with_state_mut_on<State: 'static, F: FnOnce(&mut State) + Clone>(
    win: &impl WindowExt,
    f: F,
) {
    f(state_mut!());
    app::handle(STATE_CHANGED, win).ok();
}

/// Provides read-only access to the global state for the given closure.
pub fn with_state<State: 'static, F: FnOnce(&State) + Clone>(f: F) {
    f(state_ref!());
}

/// Triggers a global UI update by emitting `STATE_CHANGED` on the main window and waking the app.
pub fn notify() {
    app::handle_main(STATE_CHANGED).ok();
    app::awake();
}

/// Triggers a UI update for the specified window.
/// Also calls `app::awake()` to wake up the event loop.
pub fn notify_win(win: &impl WindowExt) {
    app::handle(STATE_CHANGED, win).ok();
    app::awake();
}
