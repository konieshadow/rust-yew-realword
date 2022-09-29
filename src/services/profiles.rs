use crate::{types::{ProfileBody, EmptyBody}, error::Error};

use super::requests::{request_get, request_post, request_delete};

pub async fn follow(username: &str) -> Result<ProfileBody, Error> {
    request_post::<EmptyBody, ProfileBody>(&format!("/profiles/{}/follow", username), EmptyBody::default()).await
}

pub async fn unfollow(username: &str) -> Result<ProfileBody, Error> {
    request_delete::<ProfileBody>(&format!("/profiles/{}/follow", username)).await
}

pub async fn get(username: &str) -> Result<ProfileBody, Error> {
    request_get::<ProfileBody>(&format!("/profiles/{}", username)).await
}