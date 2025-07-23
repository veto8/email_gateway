//use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use axum::{
    extract::Query,
    http::{HeaderMap, Method},
    response::IntoResponse,
    Json,
};
use base64::{decode, encode};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
}

pub async fn get_token(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // http://127.0.0.1:8889/get_token
    let mut token = "".to_string();
    let mut page = "".to_string();
    if params.contains_key("page") {
        page = params["page"].replace(" ", "");
        println!("{}", page);
    }

    let host: &str = headers.get("host").unwrap().to_str().unwrap();

    let _token = encode_token(host).await;
    if _token.is_ok() {
        token = _token.unwrap();
    }

    let r = serde_json::json!([
        {
            "name": "email token",
            "host": host,
            "page": page,
            "token":token
        }
    ]);

    Json(r)
}

pub async fn encode_token(host: &str) -> Result<String, Box<dyn Error>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret-xxxx")?;
    //let mut claims = BTreeMap::new();
    let mut claims = BTreeMap::<&str, &str>::new();
    let _unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + (60 * 60 * 24 * 3650);
    let unixtime = &_unixtime.to_string();
    claims.insert("h", host);
    claims.insert("e", unixtime);
    let token = claims.sign_with_key(&key)?;

    Ok(token)
}
