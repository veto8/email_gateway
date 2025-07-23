use axum::{http::header::HeaderMap, response::IntoResponse, Extension, Json};

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
