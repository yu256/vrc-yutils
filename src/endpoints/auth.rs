use crate::{
    fetcher::{make_request, request_json, Header, ResponseExt},
    var::{Config, ConfigRW as _, CFG},
};
use axum::Json;
use hyper::Method;

#[derive(serde::Deserialize)]
pub(super) enum Query {
    Auth(AuthQuery),
    TwoFactor(TwoFactorQuery),
    Token(TokenQuery),
}

#[derive(serde::Deserialize)]
pub(super) struct AuthQuery {
    encoded: String,
}

#[derive(serde::Deserialize)]
pub(super) struct TwoFactorQuery {
    token: String,
    auth_type: String,
    two_factor_code: String,
}

#[derive(serde::Deserialize)]
pub(super) struct TokenQuery {
    token: String,
}

#[derive(serde::Serialize)]
pub(super) enum Response {
    Success,
    RequiredAuth(RequiredAuthResponse),
}

#[derive(serde::Serialize)]
pub(super) struct RequiredAuthResponse {
    token: String,
    auth_type: &'static str,
}

pub(super) async fn auth(Json(query): Json<Query>) -> Json<Response> {
    match query {
        Query::Auth(query) => auth_query(query).await,
        Query::TwoFactor(query) => two_factor_query(query).await,
        Query::Token(query) => token_query(query).await,
    }
}

async fn auth_query(AuthQuery { encoded }: AuthQuery) -> Json<Response> {
    #[allow(non_snake_case)]
    #[derive(serde::Deserialize)]
    struct TwoFactor {
        requiresTwoFactorAuth: Vec<String>,
    }

    let res = make_request(
        Method::GET,
        "https://api.vrchat.cloud/api/1/auth/user",
        Header::Auth(("Authorization", &encoded)),
        None::<()>,
    )
    .await
    .unwrap();

    let token = res
        .headers()
        .get("set-cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|c| c.split(';').next())
        .unwrap()
        .to_owned();

    let auth_type = res
        .json::<TwoFactor>()
        .await
        .unwrap()
        .requiresTwoFactorAuth
        .into_iter()
        .find_map(|auth| match auth.as_str() {
            "emailOtp" => Some("emailotp"),
            "totp" => Some("totp"),
            _ => None,
        })
        .unwrap_or("otp");

    Json(Response::RequiredAuth(RequiredAuthResponse {
        token,
        auth_type,
    }))
}

async fn two_factor_query(
    TwoFactorQuery {
        token,
        auth_type,
        two_factor_code,
    }: TwoFactorQuery,
) -> Json<Response> {
    request_json(
        Method::POST,
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{auth_type}/verify"),
        &token,
        serde_json::json!({ "code": two_factor_code }),
    )
    .await
    .unwrap();

    CFG.set(move |old| Config {
        token,
        alt_url: old.alt_url.clone(),
    })
    .await
    .unwrap();

    Json(Response::Success)
}

async fn token_query(TokenQuery { token }: TokenQuery) -> Json<Response> {
    CFG.set(move |old| Config {
        token,
        alt_url: old.alt_url.clone(),
    })
    .await
    .unwrap();

    Json(Response::Success)
}
