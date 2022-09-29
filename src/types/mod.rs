use std::collections::HashMap;

mod auth;
mod profiles;
mod articles;
mod tag;
mod comment;

pub use auth::*;
pub use profiles::*;
pub use articles::*;
pub use tag::*;
pub use comment::*;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type EmptyBody = HashMap<(), ()>;