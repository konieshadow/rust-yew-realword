use crate::{types::{UserBody, AuthUser, LoginUser, NewUser, UpdateUser}, error::Error};

use super::requests::{request_post, request_get, request_put};

pub async fn current() -> Result<UserBody<AuthUser>, Error> {
    request_get::<UserBody<AuthUser>>("/user").await
}

pub async fn login(login_user: UserBody<LoginUser>) -> Result<UserBody<AuthUser>, Error> {
    request_post::<UserBody<LoginUser>, UserBody<AuthUser>>("/users/login", login_user).await
}

pub async fn register(new_user: UserBody<NewUser>) -> Result<UserBody<AuthUser>, Error> {
    request_post::<UserBody<NewUser>, UserBody<AuthUser>>("/users", new_user).await
}

pub async fn update(update_user: UserBody<UpdateUser>) -> Result<UserBody<AuthUser>, Error> {
    request_put::<UserBody<UpdateUser>, UserBody<AuthUser>>("/user", update_user).await
}