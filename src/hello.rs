use axum::{response::IntoResponse, Json};

pub async fn hello() -> String {
    "Hallo, Rust library here!".to_string()
}

pub async fn test() -> impl IntoResponse {
    // http://127.0.0.1:8889/test

    let r = serde_json::json!([
        {
        "name": "test",
        },
    ]);

    Json(r)
}
