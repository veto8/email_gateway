use axum::{http::header::HeaderMap, response::IntoResponse, Extension, Json};

pub async fn test(headers: HeaderMap) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    println!("{:?}", headers);
    let name: &str = &env!("name");
    let host: &str = headers.get("host").unwrap().to_str().unwrap();
    let user_agent: &str = headers.get("user-agent").unwrap().to_str().unwrap();
    //let _origin: &str = headers.get("origin").unwrap().to_str().unwrap();

    let r = serde_json::json!([
        {
            "name": name,
            "host": host,
            "user_agent": user_agent,
        }
    ]);

    Json(r)
}
