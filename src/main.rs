use self::routes as r;
use silver_pos::*;

use axum::{
    http::Method,
    http::header::CONTENT_TYPE,
    response::sse::{Event, KeepAlive, Sse}, // <---
    routing::{get, post},
    Router, Extension,
};
use futures_util::stream::{self, Stream};
use sqlx::postgres::PgPoolOptions; // <---
use std::{convert::Infallible, time::Duration};
use tokio_stream::StreamExt as _; // <--- // <---

use tower_http::cors::{Any, CorsLayer};

use dotenvy::dotenv;
use std::env;

async fn sse_handler() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| Event::default().data("Hi!"))
        .map(Ok)
        .throttle(Duration::from_secs(1));
    Sse::new(stream).keep_alive(KeepAlive::default())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error connecting to database");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/categories", get(r::categories))
        .route("/items", get(r::items))
        .route("/vouchers", get(r::voucher_list))
        .route("/new_item", post(r::new_item))
        .route("/sse", get(sse_handler))
        .route("/new_cat", post(r::new_cat))
        .route("/items/:cat_id", get(r::items_by_cat))
        .route("/purchase", post(r::purchase))
        .layer(cors)
        .layer(Extension(pool));

    /*
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Axum server listening at {}", addr.to_string());
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    */

    Ok(app.into())
}

async fn root() -> &'static str {
    "Hello, World!"
}
