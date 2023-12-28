use crate::models::AppState;

use http::header::SET_COOKIE;
use oauth2::{
    basic::BasicClient,
    reqwest::async_http_client, Scope,
    AuthUrl, TokenUrl, RedirectUrl, CsrfToken,
    ClientId, ClientSecret, AuthorizationCode,
    TokenResponse
};
use axum::{
    extract::{Query, State, Extension},
    http::{StatusCode, Request, HeaderMap},
    response::{IntoResponse, Redirect, Response},
    middleware::Next
};
use axum_extra::extract::cookie::{
    Cookie, PrivateCookieJar
};
use anyhow::{Context, Result};
use chrono::{Local, Duration};
use time::Duration as TimeDuration;
use serde::Deserialize;

const AUTH_URL: &str = "https://dev-w2v52ay3bjjrf8hm.us.auth0.com/authorize?response_type=code";
const TOKEN_URL: &str = "https://dev-w2v52ay3bjjrf8hm.us.auth0.com/oauth/token";
const REDIRECT_URI: &str = "http://localhost:8000/api/auth/auth0_callback";
const API_AUDIENCE: &str = "https://dev-w2v52ay3bjjrf8hm.us.auth0.com/api/v2/";
static COOKIE_NAME: &str = "SESSION";

pub fn oauth_client(client_id: String, client_secret: String) -> BasicClient {
    let auth_url = AuthUrl::new(AUTH_URL.to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(TOKEN_URL.to_string())
        .expect("Invalid token endpoint URL");

    BasicClient::new(
        ClientId::new(client_id),
        Some(ClientSecret::new(client_secret)),
        auth_url,
        Some(token_url)
    )
    .set_redirect_uri(RedirectUrl::new(REDIRECT_URI.to_string()).unwrap())
}

pub async fn auth0_auth(Extension(oauth_client): Extension<BasicClient>) -> impl IntoResponse {
    let (auth_url, _csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .url();

    Redirect::to(auth_url.as_ref())
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String
}

#[derive(Deserialize, sqlx::FromRow, Clone)]
pub struct UserProfile {
    email: String
}

pub async fn auth0_callback(
    State(state): State<AppState>,
    Query(query): Query<AuthRequest>,
    Extension(oauth_client): Extension<BasicClient>
) -> Result<impl IntoResponse, AppError> {
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await
        .context("Failed in sending request to authorization server")?;

    let client = reqwest::Client::new();
    let profile: UserProfile = client
        .get(format!("{}{}", API_AUDIENCE, "userinfo"))
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("Failed in sending request to target Url")?
        .json::<UserProfile>()
        .await
        .context("Failed to deserialize response as JSON")?;

    let secs: i64 = token.expires_in().unwrap().as_secs().try_into().unwrap();

    let max_age = (Local::now().naive_local() + Duration::seconds(secs)).and_utc();

    // TODO: this statement get the email from another table check against current table and do
    // insertion as necessary. Try to do the same thing on your conditional endpoints
    sqlx::query!(r#"INSERT INTO sessions (user_id, session_id, expires_at) VALUES (
        (SELECT ID FROM USERS WHERE email = $1 LIMIT 1),
        $2, $3)
        ON CONFLICT (user_id) DO UPDATE SET
        session_id = excluded.session_id,
        expires_at = excluded.expires_at"#,
        profile.email,
        token.access_token().secret().to_owned(),
        max_age)
        .execute(&state.db)
        .await
        .context("Error trying to create sessions")?;

    sqlx::query!("INSERT INTO users (email) VALUES ($1) ON CONFLICT (email) DO NOTHING", profile.email.clone())
        .execute(&state.db)
        .await
        .context("Error trying to create account!")?;

    let cookie = Cookie::build((COOKIE_NAME, token.access_token().secret().to_owned()))
        .domain("localhost:8080")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(TimeDuration::seconds(secs))
        .build();

    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.to_string().parse().unwrap()
    );
    Ok((headers, Redirect::to("/")))
}

pub async fn check_authenticated<B>(
    State(state): State<AppState>,
    jar: PrivateCookieJar,
    mut req: Request<B>,
    next: Next<B>
) -> Result<impl IntoResponse, impl IntoResponse> {
    let Some(cookie) = jar.get(COOKIE_NAME).map(|cookie| cookie.value().to_owned()) else {
        return Err((StatusCode::UNAUTHORIZED, "Unauthorized!".to_string()));
    };

    let res: UserProfile = match sqlx::query_as!(UserProfile, "SELECT
        users.email
        FROM sessions
        LEFT JOIN USERS ON sessions.user_id = users.id
        WHERE sessions.session_id = $1
        LIMIT 1",
        cookie)
        .fetch_one(&state.db)
        .await {
            Ok(res) => res,
            Err(e) => {
                return Err((StatusCode::UNAUTHORIZED, e.to_string()));
            }
        };

    let user = UserProfile {
        email: res.email
    };

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}

pub async fn protected(
    Extension(user): Extension<UserProfile>
) -> impl IntoResponse {
    (StatusCode::OK, user.email)
}

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

impl<E> From<E> for AppError
    where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
