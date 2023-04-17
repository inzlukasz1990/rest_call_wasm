// src/lib.rs
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Window, Headers};

#[wasm_bindgen]
pub async fn request(
    method: &str,
    url: &str,
    data: Option<String>,
    token: Option<String>,
    content_type: Option<String>,
) -> Result<JsValue, JsValue> {
    make_request(method, url, data, token, content_type).await
}

#[wasm_bindgen]
pub async fn get(url: &str, token: Option<String>) -> Result<JsValue, JsValue> {
    make_request("GET", url, None, token, None).await
}

#[wasm_bindgen]
pub async fn post(url: &str, data: String, token: Option<String>, content_type: Option<String>) -> Result<JsValue, JsValue> {
    make_request("POST", url, Some(data), token, content_type).await
}

#[wasm_bindgen]
pub async fn put(url: &str, data: String, token: Option<String>, content_type: Option<String>) -> Result<JsValue, JsValue> {
    make_request("PUT", url, Some(data), token, content_type).await
}

#[wasm_bindgen]
pub async fn delete(url: &str, token: Option<String>) -> Result<JsValue, JsValue> {
    make_request("DELETE", url, None, token, None).await
}

async fn make_request(
    method: &str,
    url: &str,
    data: Option<String>,
    token: Option<String>,
    content_type: Option<String>,
) -> Result<JsValue, JsValue> {
    let window = web_sys::window().expect("No global `window` exists");
    let request = Request::new_with_str_and_init(url, &request_init(method, data, token, content_type)?)
        .map_err(|_| "Failed to create request")?;

    let response = JsFuture::from(window.fetch_with_request(&request)).await?;
    Ok(response)
}

fn request_init(
    method: &str,
    data: Option<String>,
    token: Option<String>,
    content_type: Option<String>,
) -> Result<RequestInit, JsValue> {
    let mut opts = RequestInit::new();
    opts.method(method);
    opts.mode(RequestMode::Cors);

    if let Some(body) = data {
        opts.body(Some(&JsValue::from_str(&body)));
    }

    let headers = Headers::new().unwrap();
    if let Some(token) = token {
        headers.append("Authorization", &format!("Bearer {}", token)).unwrap();
    }

    if let Some(content_type) = content_type {
        headers.append("Content-Type", &content_type).unwrap();
    } else {
        headers.append("Content-Type", "application/json").unwrap();
    }

    opts.headers(&headers);

    Ok(opts)
}

