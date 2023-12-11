use self::routes as r;
use silver_pos::*;

use axum::{
    http::Method,
    http::header::CONTENT_TYPE,
    routing::{get, post},
    Router
};

use tower_http::cors::{Any, CorsLayer};

#[shuttle_runtime::main]
pub async fn axum (
    #[shuttle_shared_db::Postgres] pool: sqlx::PgPool,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Migration failed :(");

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = Router::new()
        .route("/", get(root))
        .route("/categories", get(r::categories))
        .route("/items", get(r::items))
        .route("/vouchers", get(r::voucher_list))
        .route("/partial_vouchers", get(r::partial_vouchers))
        .route("/cashflows", get(r::cash_flow_list))
        .route("/new_item", post(r::new_item))
        .route("/new_cat", post(r::new_cat))
        .route("/new_cashflow", post(r::new_cash_flow))
        .route("/items/:cat_id", get(r::items_by_cat))
        .route("/vouchers/:id", get(r::voucher_by_id))
        .route("/purchase", post(r::purchase))
        .layer(cors)
        .with_state(pool);

    Ok(app.into())
}

/*
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
}*/

async fn root() -> &'static str {
    "Hello, World!"
}
