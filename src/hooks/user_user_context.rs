use std::{ops::Deref, fmt::Formatter};
use std::fmt::Debug;
use yew::{UseStateHandle, use_context};
use yew_router::prelude::{AnyHistory, History, use_history};

use crate::{types::AuthUser, services::requests::set_token, routes::AppRoute};

pub struct UseUserContextHandle {
    inner: UseStateHandle<AuthUser>,
    history: AnyHistory,
}

impl UseUserContextHandle {
    pub fn login(&self, value: AuthUser) {
        set_token(Some(value.token.clone()));
        self.inner.set(value);
        self.history.push(AppRoute::Home);
    }

    pub fn logout(&self) {
        set_token(None);
        self.inner.set(AuthUser::default());
        self.history.push(AppRoute::Home);
    }
}

impl Deref for UseUserContextHandle {
    type Target = AuthUser;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

impl Debug for UseUserContextHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UseUserContextHandle")
            .field("value", &format!("{:?}", *self.inner))
            .finish()
    }
}

pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<AuthUser>>().unwrap();
    let history = use_history().unwrap();

    UseUserContextHandle { inner, history }
}