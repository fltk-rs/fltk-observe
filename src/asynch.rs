#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fltk::{
    app,
    enums::{Event, Shortcut},
    menu::MenuFlag,
    prelude::*,
};
use std::any::Any;
use std::sync::{Arc, LazyLock};
use tokio::sync::Mutex;

pub const STATE_CHANGED: Event = Event::from_i32(100);

static STATE: LazyLock<Mutex<Option<Box<dyn Any + 'static + Send>>>> =
    LazyLock::new(|| Mutex::new(None));

pub trait WidgetObserver<T: 'static, W> {
    fn set_action<
        Fut: Send + Future<Output = ()> + 'static,
        Listen: Sync + Send + Clone + 'static + Fn(Arc<T>, Arc<Self>) -> Fut,
    >(
        &mut self,
        l: Listen,
    ) where
        T: Clone,
        Self: Sized;
    fn set_action_sync<Listen: Clone + 'static + Fn(&mut T, &Self)>(&mut self, l: Listen);
    fn set_view<
        Fut: Send + Future<Output = ()> + 'static,
        Update: Send + Sync + Clone + 'static + Fn(Arc<T>, Self) -> Fut,
    >(
        &mut self,
        u: Update,
    ) where
        T: Clone,
        Self: Sized;
    fn set_view_sync<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update);
}

pub trait MenuObserver<T, W> {
    fn add_action<
        Fut: Send + Future<Output = ()> + 'static,
        Listen: Sync + Send + Clone + 'static + Fn(Arc<T>, Arc<Self>) -> Fut,
    >(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flags: MenuFlag,
        l: Listen,
    ) where
        T: Clone,
        Self: Sized;
    fn add_action_sync<Listen: Clone + 'static + Fn(&mut T, &Self)>(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flags: MenuFlag,
        l: Listen,
    );
}

impl<T: Send + Sync + 'static, W: WidgetExt + WidgetBase + 'static + Clone + Send>
    WidgetObserver<T, W> for W
{
    fn set_action<
        Fut: Send + Future<Output = ()> + 'static,
        Listen: Send + Sync + Clone + 'static + Fn(Arc<T>, Arc<Self>) -> Fut,
    >(
        &mut self,
        l: Listen,
    ) where T: Clone,
    {
        self.set_callback(move |w| {
            let w = w.clone();
            let l = l.clone();
            tokio::spawn(async move {
                let w = w.clone();
                let l = l.clone();
                l(
                    Arc::new(
                        STATE
                            .lock()
                            .await
                            .as_mut()
                            .unwrap()
                            .downcast_mut::<T>()
                            .unwrap()
                            .clone(),
                    ),
                    w.into(),
                )
                .await;
                app::handle_main(STATE_CHANGED).ok();
            });
        });
    }

    fn set_action_sync<Listen: Clone + 'static + Fn(&mut T, &Self)>(&mut self, l: Listen) {
        self.set_callback(move |w| {
            let w = w.clone();
            let l = l.clone();
            l(
                STATE
                    .blocking_lock()
                    .as_mut()
                    .unwrap()
                    .downcast_mut()
                    .unwrap(),
                &w,
            );
            app::handle_main(STATE_CHANGED).ok();
        });
    }

    fn set_view<
        Fut: Send + Future<Output = ()> + 'static,
        Update: Send + Sync + Clone + 'static + Fn(Arc<T>, Self) -> Fut,
    >(
        &mut self,
        u: Update,
    ) where
    T: Clone, 
    {
        let w = self.clone();
        let func = move || {
            let w = w.clone();
            let u = u.clone();
            tokio::spawn(async move {
                u(
                    Arc::new(
                        STATE
                            .lock()
                            .await
                            .as_mut()
                            .unwrap()
                            .downcast_mut::<T>()
                            .unwrap()
                            .clone(),
                    ),
                    w,
                )
                .await;
                app::awake();
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

    fn set_view_sync<Update: Clone + 'static + Fn(&T, &mut Self)>(&mut self, u: Update) {
        let w = self.clone();
        let func = move || {
            let mut w = w.clone();
            let u = u.clone();
            u(
                STATE
                    .blocking_lock()
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

impl<T: Send + Sync + 'static, W: MenuExt + 'static + Clone + Send> MenuObserver<T, W>
    for W
{
    fn add_action<
        Fut: Send + Future<Output = ()> + 'static,
        Listen: Sync + Send + Clone + 'static + Fn(Arc<T>, Arc<Self>) -> Fut,
    >(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flags: MenuFlag,
        l: Listen,
    ) where T: Clone,
    {
        self.add(label, shortcut, flags, move |w| {
            let w = w.clone();
            let l = l.clone();
            tokio::spawn(async move {
                let w = w.clone();
                let l = l.clone();
                l(
                    Arc::new(
                        STATE
                            .lock()
                            .await
                            .as_mut()
                            .unwrap()
                            .downcast_mut::<T>()
                            .unwrap()
                            .clone(),
                    ),
                    w.into(),
                )
                .await;
                app::handle_main(STATE_CHANGED).ok();
            });
        });
    }

    fn add_action_sync<Listen: Clone + 'static + Fn(&mut T, &Self)>(
        &mut self,
        label: &str,
        shortcut: Shortcut,
        flags: MenuFlag,
        l: Listen,
    ) {
        self.add(label, shortcut, flags, move |w| {
            let w = w.clone();
            let l = l.clone();
            l(
                STATE
                    .blocking_lock()
                    .as_mut()
                    .unwrap()
                    .downcast_mut()
                    .unwrap(),
                &w,
            );
            app::handle_main(STATE_CHANGED).ok();
        });
    }
}

pub trait Runner<State: 'static + Send + Sync> {
    fn with_state<F: Send + 'static + FnMut() -> State>(
        self,
        init: F,
    ) -> impl std::future::Future<Output = Self> + Send
    where
        Self: Sized;
}

impl<State: 'static + Send + Sync> Runner<State> for app::App {
    async fn with_state<F: Send + 'static + FnMut() -> State>(self, mut init: F) -> Self
    where
        Self: Sized,
    {
        *STATE.lock().await = Some(Box::new((init)()));
        self
    }
}

pub fn use_state_mut<
    Fut: Send + 'static + Future<Output = ()>,
    State: 'static + Clone,
    F: (FnMut(Arc<State>) -> Fut) + Clone + Send + 'static,
>(
    f: F,
) {
    let mut f = f.clone();
    tokio::spawn(async move {
        f(Arc::new(
            STATE
                .lock()
                .await
                .as_mut()
                .unwrap()
                .downcast_mut::<State>()
                .unwrap()
                .clone(),
        ))
        .await;
        app::handle_main(STATE_CHANGED).ok();
    });
}

pub fn use_state<
    Fut: Send + 'static + Future<Output = ()>,
    State: 'static + Clone,
    F: (FnMut(Arc<State>) -> Fut) + Clone + Send + 'static,
>(
    f: F,
) {
    let mut f = f.clone();
    tokio::spawn(async move {
        f(Arc::new(
            STATE
                .lock()
                .await
                .as_ref()
                .unwrap()
                .downcast_ref::<State>()
                .unwrap()
                .clone(),
        ))
        .await;
    });
}


pub fn use_state_sync_mut<State: 'static, F: FnMut(&mut State) + Clone>(f: F) {
    let mut f = f.clone();
    f(STATE.blocking_lock().as_mut().unwrap().downcast_mut().unwrap());
    app::handle_main(STATE_CHANGED).ok();
}

pub fn use_state_sync<State: 'static, F: FnMut(&State) + Clone>(f: F) {
    let mut f = f.clone();
    f(STATE.blocking_lock().as_ref().unwrap().downcast_ref().unwrap());
}
