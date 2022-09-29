use crate::{error::Error, types::{MultipleArticlesBody, ArticleBody, CreateUpdateArticle, EmptyBody}};

use super::requests::{request_get, limit, request_delete, request_post, request_put};

pub async fn all(page: u32) -> Result<MultipleArticlesBody, Error> {
    request_get::<MultipleArticlesBody>(&format!("/articles?{}", limit(10, page))).await
}

pub async fn by_author(author: &str, page: u32) -> Result<MultipleArticlesBody, Error> {
    request_get::<MultipleArticlesBody>(&format!("/articles?author={}&{}", author, limit(10, page))).await
}

pub async fn by_tag(tag: &str, page: u32) -> Result<MultipleArticlesBody, Error> {
    request_get::<MultipleArticlesBody>(&format!("/articles?tag={}&{}", tag, limit(10, page))).await
}

pub async fn del(slug: &str) -> Result<EmptyBody, Error> {
    request_delete::<EmptyBody>(&format!("/articles/{}", slug)).await
}

pub async fn favorite(slug: &str) -> Result<ArticleBody, Error> {
    request_post::<EmptyBody, ArticleBody>(&format!("/articles/{}/favorite", slug), EmptyBody::default()).await
}

pub async fn unfavorite(slug: &str) -> Result<ArticleBody, Error> {
    request_delete::<ArticleBody>(&format!("/articles/{}/favorite", slug)).await
}

pub async fn favorited_by(author: &str, page: u32) -> Result<MultipleArticlesBody, Error> {
    request_get::<MultipleArticlesBody>(&format!(
        "/articles?favorited={}&{}",
        author,
        limit(10, page)
    ))
    .await
}

pub async fn feed() -> Result<MultipleArticlesBody, Error> {
    request_get::<MultipleArticlesBody>(&format!("/articles/feed?{}", limit(10, 0))).await
}

pub async fn get(slug: &str) -> Result<ArticleBody, Error> {
    request_get::<ArticleBody>(&format!("/articles/{}", slug)).await
}

pub async fn update(slug: &str, article: ArticleBody<CreateUpdateArticle>) -> Result<ArticleBody, Error> {
    request_put::<ArticleBody<CreateUpdateArticle>, ArticleBody>(&format!("/articles/{}", slug), article).await
}

pub async fn create(article: ArticleBody<CreateUpdateArticle>) -> Result<ArticleBody, Error> {
    request_post::<ArticleBody<CreateUpdateArticle>, ArticleBody>("/articles", article).await
}