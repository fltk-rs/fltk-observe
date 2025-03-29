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
        Self: Sized;
    fn set_view<
        Fut: Send + Future<Output = ()> + 'static,
        Update: Send + Sync + Clone + 'static + Fn(Arc<T>, Self) -> Fut,
    >(
        &mut self,
        u: Update,
    ) where
        Self: Sized;
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
        Self: Sized;
}

impl<T: Send + Sync + 'static + Clone, W: WidgetExt + WidgetBase + 'static + Clone + Send>
    WidgetObserver<T, W> for W
{
    fn set_action<
        Fut: Send + Future<Output = ()> + 'static,
        Listen: Send + Sync + Clone + 'static + Fn(Arc<T>, Arc<Self>) -> Fut,
    >(
        &mut self,
        l: Listen,
    ) {
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

    fn set_view<
        Fut: Send + Future<Output = ()> + 'static,
        Update: Send + Sync + Clone + 'static + Fn(Arc<T>, Self) -> Fut,
    >(
        &mut self,
        u: Update,
    ) {
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
        self.handle(move |_w, ev| {
            if ev == STATE_CHANGED {
                func();
            }
            false
        });
    }
}

impl<T: Send + Sync + 'static + Clone, W: MenuExt + 'static + Clone + Send> MenuObserver<T, W>
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
    ) {
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

pub async fn use_state_mut<State: 'static, F: FnMut(&mut State) + Clone>(f: F) {
    let mut f = f.clone();
    f(STATE.lock().await.as_mut().unwrap().downcast_mut().unwrap());
    app::handle_main(STATE_CHANGED).ok();
}

pub async fn use_state<State: 'static, F: FnMut(&State) + Clone>(f: F) {
    let mut f = f.clone();
    f(STATE.lock().await.as_ref().unwrap().downcast_ref().unwrap());
}
