use crate::{types::TagsBody, error::Error};

use super::requests::request_get;

pub async fn get_all() -> Result<TagsBody, Error> {
    request_get::<TagsBody>("/tags").await
}