use crate::{
    error::Error,
    types::{AddComment, CommentBody, MultipleCommentsBody, EmptyBody},
};

use super::requests::{request_post, request_delete, request_get};

pub async fn create(slug: &str, comment: CommentBody<AddComment>) -> Result<CommentBody, Error> {
    request_post::<CommentBody<AddComment>, CommentBody>(
        &format!("/articles/{}/comments", slug),
        comment,
    )
    .await
}

pub async fn delete(slug: &str, comment_id: u32) -> Result<EmptyBody, Error> {
    request_delete::<EmptyBody>(&format!("/articles/{}/comments/{}", slug, comment_id)).await
}

pub async fn for_article(slug: &str) -> Result<MultipleCommentsBody, Error> {
    request_get::<MultipleCommentsBody>(&format!("/articles/{}/comments", slug)).await
}