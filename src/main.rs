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
        .route("/delete_cat/:id", post(r::delete_cat))
        .route("/delete_item/:id", post(r::delete_item))
        .route("/add_stock", post(r::add_stock))
        .layer(cors)
        .with_state(pool);

    Ok(app.into())
}

async fn root() -> &'static str {
    "Hello, SilverLight!"
}
