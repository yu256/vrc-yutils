use crate::fetcher::{make_request, request_json, Header, ResponseExt as _};
use anyhow::Context as _;
use base64::{engine::general_purpose, Engine as _};
use hyper::Method;
use serde_json::json;
use std::io;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct TwoFactor {
    requiresTwoFactorAuth: Vec<String>,
}

pub(crate) async fn auth() -> anyhow::Result<String> {
    let mut username = String::new();
    let mut password = String::new();

    println!("Enter your username or email:");
    io::stdin().read_line(&mut username)?;

    println!("Enter your password:");
    io::stdin().read_line(&mut password)?;

    let res = make_request(
        Method::GET,
        URL,
        Header::Auth((
            "Authorization",
            &format!(
                "Basic {}",
                general_purpose::STANDARD_NO_PAD.encode(format!(
                    "{}:{}",
                    username.trim(),
                    password.trim()
                ))
            ),
        )),
        None::<()>,
    )
    .await?;

    let token = res
        .headers()
        .get("set-cookie")
        .and_then(|h| h.to_str().ok())
        .and_then(|c| c.split(';').next())
        .context("invalid cookie found.")?
        .to_owned();

    let auth_type = res
        .json::<TwoFactor>()
        .await?
        .requiresTwoFactorAuth
        .into_iter()
        .find_map(|auth| match auth.as_str() {
            "emailOtp" => Some("emailotp"),
            "totp" => Some("totp"),
            _ => None,
        })
        .unwrap_or("otp");

    let mut two_factor_code = String::new();

    println!("Enter your code:");
    io::stdin().read_line(&mut two_factor_code)?;

    request_json(
        Method::POST,
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{auth_type}/verify"),
        &token,
        json!({ "code": two_factor_code.trim() }),
    )
    .await?;

    let token = token.split_once('=').unwrap().1.to_owned();

    println!("your token: {token}\n次回から--auth={token}として起動してください。");

    Ok(token)
}
