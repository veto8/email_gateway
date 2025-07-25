use crate::token;
use axum::{extract, http::header::HeaderMap, response::IntoResponse, Extension, Json};
use urldecode::decode;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Message {
    xemail: String,
    xname: String,
    xmessage: String,
    token: String,
    random: String,
}

pub async fn email(extract::Json(payload): extract::Json<Message>) {
    let name = decode(payload.xname);
    let email = decode(payload.xemail);
    let message = decode(payload.xmessage);
    let token = decode(payload.token);
    let random = decode(payload.random);
    let v_message: Vec<String> = serde_json::from_str(&message).unwrap();
    let x = token::test_ok().await;
    let a = token::auth_token(&token).await;
    println!("{:?}", a);
}

pub async fn emailx(headers: HeaderMap) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    println!("{:?}", headers);
    let host: &str = headers.get("host").unwrap().to_str().unwrap();

    let r = serde_json::json!([
        {
            "name": "test",
            "host": host,
        }
    ]);

    Json(r)
}
