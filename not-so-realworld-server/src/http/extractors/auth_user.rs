use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{header::AUTHORIZATION, request::Parts, HeaderValue},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    config::Config,
    http::{response::ApiError, Result, DEFAULT_SESSION_LENGTH, SCHEME_PREFIX},
};

pub struct AuthUser(pub Uuid);

pub struct AuthUserToken(pub Uuid, pub String);

#[derive(Serialize, Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    exp: i64,
}

// ===== AuthUserToken =====

impl AuthUserToken {
    fn from_authorization(config: &Config, header: &HeaderValue) -> Result<Self> {
        let auth_header = header.to_str().map_err(|_| ApiError::Unauthorized)?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            return Err(ApiError::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];
        let key =
            DecodingKey::from_base64_secret(&config.hmac_key).expect("error creating decoding key");
        let token_message =
            jsonwebtoken::decode::<AuthUserClaims>(token, &key, &Validation::default())
                .map_err(|_| ApiError::Unauthorized)?;

        if token_message.claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            return Err(ApiError::Unauthorized);
        }

        Ok(AuthUserToken(
            token_message.claims.user_id,
            token.to_string(),
        ))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUserToken
where
    Config: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = Config::from_ref(state);
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(ApiError::Unauthorized)?;

        AuthUserToken::from_authorization(&config, &header)
    }
}

// ===== AuthUser =====

impl AuthUser {
    pub fn to_jwt(&self, config: &Config) -> String {
        let exp = (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp();
        let claims = AuthUserClaims {
            user_id: self.0,
            exp,
        };
        let header = Header::default();
        let key =
            EncodingKey::from_base64_secret(&config.hmac_key).expect("error creating encoding key");

        jsonwebtoken::encode(&header, &claims, &key).expect("error signing jwt")
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    Config: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let config = Config::from_ref(state);
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(ApiError::Unauthorized)?;

        Ok(AuthUser(
            AuthUserToken::from_authorization(&config, &header)?.0,
        ))
    }
}
