#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::{
    app,
    enums::{Event, Shortcut},
    menu::MenuFlag,
    prelude::*,
};
use std::any::Any;
use std::cell::RefCell;

pub const STATE_CHANGED: Event = Event::from_i32(100);

thread_local! {
    static STATE: RefCell<Option<Box<dyn Any>>> = RefCell::new(None);
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
            let w = w.clone();
            let l = l.clone();
            STATE.with(move |s| {
                l(s.borrow_mut().as_mut().unwrap().downcast_mut().unwrap(), &w);
                app::handle_main(STATE_CHANGED).ok();
            });
        });
    }

    fn set_view<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update) {
        let w = self.clone();
        let func = move || {
            STATE.with({
                let mut w = w.clone();
                let u = u.clone();
                move |s| {
                    u(s.borrow().as_ref().unwrap().downcast_ref().unwrap(), &mut w);
                }
            });
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
            let w = w.clone();
            let l = l.clone();
            STATE.with(move |s| {
                l(s.borrow_mut().as_mut().unwrap().downcast_mut().unwrap(), &w);
                app::handle_main(STATE_CHANGED).ok();
            });
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
        STATE.with(|s| *s.borrow_mut() = Some(Box::new((init)())));
        self
    }
}

pub fn use_state_mut<State: 'static, F: FnMut(&mut State) + Clone>(f: F) {
    let mut f = f.clone();
    STATE.with(|s| {
        f(s.borrow_mut().as_mut().unwrap().downcast_mut().unwrap());
        app::handle_main(STATE_CHANGED).ok();
    });
}

pub fn use_state<State: 'static, F: FnMut(&State) + Clone>(f: F) {
    let mut f = f.clone();
    STATE.with(|s| {
        f(s.borrow().as_ref().unwrap().downcast_ref().unwrap());
    });
}
