use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use axum::{async_trait, RequestPartsExt};
use lazy_regex::regex_captures;
use tower_cookies::Cookies;

use crate::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::{Error, Result};

pub async fn mw_require_auth(cookies: Cookies, req: Request<Body>, next: Next) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");
    let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string()); // Get auth-token cookie.

    // Parse token.
    let (user_id, exp, sign) = auth_token
        .ok_or(Error::AuthFailNoAuthTokenCookie) // If no auth-token cookie, return error.
        .and_then(parse_token)?; // Parse token after checking if it exists.

    // TODO: Token components validation.

    Ok(next.run(req).await)
}

// Create a custom extractor for Ctx.
// region: --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        println!("->> {:<12} - Ctx", "EXTRACTOR");

        // User the cookies extractor.
        let cookies = parts.extract::<Cookies>().await.unwrap();

        let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string()); // Get auth-token cookie.

        // Parse token.
        let (user_id, exp, sign) = auth_token
            .ok_or(Error::AuthFailNoAuthTokenCookie) // If no auth-token cookie, return error.
            .and_then(parse_token)?; // Parse token after checking if it exists.

        // TODO: Token components validation.

        Ok(Ctx::new(user_id))
    }
}
// endregion: --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
    let (_whole, user_id, exp, sign) = regex_captures!(
        r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
        &token
    )
    .ok_or(Error::AuthFailTokenWrongFormat)?;

    // Parse user_id to u64
    let user_id: u64 = user_id
        .parse()
        .map_err(|_| Error::AuthFailTokenWrongFormat)?;

    Ok((user_id, exp.to_string(), sign.to_string()))
}
