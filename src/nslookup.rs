use axum::{
    extract::{ConnectInfo, Request},
    http::header::HeaderMap,
    response::IntoResponse,
    Extension, Json,
};
use axum_client_ip::XRealIp as ClientIp;
use std::net::SocketAddr;
use std::process::Command;

pub async fn nslookup(headers: HeaderMap, req: Request) -> impl IntoResponse {
    // http://127.0.0.1:8889/test
    //println!("{:?}", ip);

    let output = Command::new("sh")
        .arg("-c")
        .arg("nslookup -type=txt baeckerei-katz.de")
        .output()
        .expect("Failed to execute command");

    let r = serde_json::json!([
        {
            "name": String::from_utf8_lossy(output.stdout.as_slice()),

        }
    ]);

    Json(r)
}
