use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use libs::hello::*;
use libs::sign_in::*;

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let token = encode_token().await;
    let is_auth = auth_token(&token.unwrap()).await.unwrap();
    println!("{}", is_auth);

    let cors = CorsLayer::new()
        .allow_origin("http://127.0.0.1".parse::<HeaderValue>().unwrap())
        .allow_origin(
            "https://domain-swapper.myridia.com"
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_methods([Method::GET, Method::OPTIONS, Method::POST]);

    let app = Router::new()
        .route("/test", get(test))
        .route("/sign_in", get(sign_in))
        .layer(cors)
        .layer(CorsLayer::permissive());

    println!("Server started successfully");
    let host = "0.0.0.0:8089";
    println!("http://{}/test", host);
    println!("http://127.0.0.1:8089/test");
    println!("https://email.local/test");
    println!("http://88.198.49.173:8089/test");
    println!("https://api.grallator.com/test");
    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
