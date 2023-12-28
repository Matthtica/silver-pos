use self::routes as r;
use self::authorization as auth;
use silver_pos::*;
use silver_pos::models::AppState;

use axum::{
    http::Method,
    http::header::CONTENT_TYPE,
    routing::{get, post},
    Router,
    Extension
};

use shuttle_secrets::SecretStore;
use tower_http::cors::{Any, CorsLayer};
use axum_extra::extract::cookie::Key;

#[shuttle_runtime::main]
pub async fn axum (
    #[shuttle_shared_db::Postgres] db: sqlx::PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!().run(&db).await.expect("Migration failed :(");

    let oauth_id = secrets.get("AUTH0_OAUTH_CLIENT_ID").unwrap();
    let oauth_secret = secrets.get("AUTH0_OAUTH_CLIENT_SECRET").unwrap();

    let state = AppState {
        db,
        key: Key::generate()
    };

    let oauth_client = auth::oauth_client(oauth_id, oauth_secret);

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let routes = Router::new()
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
        .route("/add_stock", post(r::add_stock));

    let auth_route = Router::new()
        .route("/auth/auth0_callback", get(auth::auth0_callback));

    /*
    let protected_router = Router::new()
        .route("/", get(auth::protected))
        .route_layer(middleware::from_fn_with_state(state.clone(), auth::check_authenticated));
    */

    let app = Router::new()
        .nest("/api", auth_route)
        //.nest("/protected", protected_router)
        .nest("/", routes)
        .layer(Extension(oauth_client))
        .layer(cors)
        .with_state(state);

    Ok(app.into())
}

async fn root() -> &'static str {
    "Hello, SilverLight!"
}
