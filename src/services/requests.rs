use gloo::console::error;
use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::lock_api::RwLock;
use parking_lot::RawRwLock;
use serde::de::DeserializeOwned;

use crate::error::Error;
use crate::types::ErrorInfo;

const API_ROOT: &str = dotenvy_macro::dotenv!("API_ROOT");
const TOKEN_KEY: &str = "app.token";

lazy_static! {
    pub static ref TOKEN: RwLock<RawRwLock, Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t).expect("failed to set");
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read();
    token_lock.clone()
}

pub async fn request<B, T>(method: reqwest::Method, url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static,
    B: serde::Serialize,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT;
    let url = format!("{}{}", API_ROOT, url);
    let mut builder = reqwest::Client::new()
        .request(method, url);

    if let Some(token) = get_token() {
        builder = builder.bearer_auth(token);
    }
    
    if allow_body {
        builder = builder.header("Content-Type", "application/json");
        builder = builder.json(&body);
    }

    let response = builder.send().await;

    if let Ok(data) = response {
        if data.status().is_success() {
            let content_type = data.headers().get("content-type");
            match content_type {
                Some(content_type) if content_type.to_str().unwrap().starts_with("application/json") => {
                    data.json::<T>().await.map_err(|_| Error::DeserializeError)
                },
                _ => {
                    serde_json::from_str::<T>("{}").map_err(|_| Error::DeserializeError)
                }
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data = data.json::<ErrorInfo>().await;
                    match data {
                        Ok(data) => Err(Error::UnprocessableEntity(data)),
                        Err(err) => {
                            error!("Request deserialize error", err.to_string());
                            Err(Error::DeserializeError)
                        }
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}

pub async fn request_delete<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned + 'static,
{
    request(reqwest::Method::DELETE, url, ()).await
}

pub async fn request_get<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned + 'static,
{
    request(reqwest::Method::GET, url, ()).await
}

pub async fn request_post<B, T>(url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static,
    B: serde::Serialize,
{
    request(reqwest::Method::POST, url, body).await
}

pub async fn request_put<B, T>(url: &str, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static,
    B: serde::Serialize,
{
    request(reqwest::Method::PUT, url, body).await
}

pub fn limit(count: u32, p: u32) -> String {
    let offset = if p > 0 { p * count } else { 0 };
    format!("limit={}&offset={}", count, offset)
}