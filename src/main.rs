use silver_pos::*;
use self::routes::{
    categories, 
    items,
    new_item
};

use axum::{
    routing::{get, post},
    http::Method,
    Router,
};
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};
use http::header::CONTENT_TYPE;

#[tokio::main]
async fn main() {

    //let connection = &mut establish_connection();
    //Mistake
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/categories", get(categories))
        .route("/items", get(items))
        .route("/new_item", post(new_item))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum server listening at {}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
