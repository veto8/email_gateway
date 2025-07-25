use axum::{http::header::HeaderMap, response::IntoResponse, Extension, Json};

#[derive(Deserialize)]
struct CreateUser {
    email: String,
    password: String,
}

async fn testx(Json(payload): Json<CreateUser>) {
    // ...
}

pub async fn test(headers: HeaderMap) -> impl IntoResponse {
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

pub async fn token(headers: HeaderMap) -> impl IntoResponse {
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
