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

pub async fn sign_in(headers: HeaderMap) -> impl IntoResponse {
    let mut token = "".to_string();

    let auth = headers["authorization"].to_str().unwrap();
    let _a = auth.split(" ");
    let a: Vec<&str> = _a.collect();
    if a.len() == 2 {
        //println!("{:?}", a);
        let v: Vec<u8> = decode(a[1]).unwrap();
        let s = String::from_utf8_lossy(&v);
        let _up = s.split(":");
        let up: Vec<&str> = _up.collect();
        let user = up[0];
        let password = up[1];
        if user == "info@foo.com" && password == "12345" {
            println!("{user} : {password}");

            /*
                let _token = encode_token().await;
                if _token.is_ok() {
                    token = _token.unwrap();
            }
                */
        }
    }
    token.into_response()
}
/*
pub async fn encode_token() -> Result<String, Box<dyn Error>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret-xxxx")?;
    //let mut claims = BTreeMap::new();
    let mut claims = BTreeMap::<&str, &str>::new();
    let _unixtime = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + (60 * 60 * 24 * 3650);
    let unixtime = &_unixtime.to_string();
    claims.insert("u", "someone");
    claims.insert("e", unixtime);
    let token = claims.sign_with_key(&key)?;

    Ok(token)
}
*/
pub async fn auth_token(token: &str) -> Result<bool, Box<dyn Error>> {
    //println!("{}", token);
    let mut r = false;
    let unix_now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret-xxxx").unwrap();
    let _claims: Result<BTreeMap<String, String>, jwt::Error> = token.verify_with_key(&key);
    if _claims.is_ok() {
        let claims = _claims.unwrap();
        let _unix_expire: Result<String, _> = claims["e"].parse();
        if _unix_expire.is_ok() {
            let unix_expire: u64 = _unix_expire.unwrap().parse().unwrap();
            if unix_expire >= unix_now {
                //println!("{}(expire time) >= {}(time now)", unix_expire, unix_now);
                r = true;
            }
        }
    }
    Ok(r)
}

pub async fn auth_header(headers: HeaderMap) -> Result<bool, Box<dyn Error>> {
    let mut r = false;

    if headers.contains_key("authorization") {
        let auth = headers["authorization"].to_str().unwrap();
        let _a = auth.split(" ");
        let a: Vec<&str> = _a.collect();
        if a.len() == 2 {
            if a[0] == "Bearer" {
                let token = auth_token(a[1]).await?;
                if token {
                    r = true;
                }
            }
        }
    }

    Ok(r)
}
