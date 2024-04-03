use anyhow::Context;
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::WithRejection;
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

use crate::http::{
    extractors::AuthUser, response::ApiError, response::ApiResponse, traits::ResultExt, AppState,
    Result,
};

use super::extractors::AuthUserToken;

#[derive(Debug, Serialize)]
pub struct User {
    email: String,
    username: String,
    token: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    username: String,
    password: String,
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
struct UpdateUser {
    username: Option<String>,
    email: Option<String>,
    password: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_current_user).post(create_user).patch(update_user),
        )
        .route("/login", post(login_user))
}

async fn create_user(
    State(state): State<AppState>,
    WithRejection(Json(req_user), _): WithRejection<Json<NewUser>, ApiError>,
) -> Result<ApiResponse<User>> {
    let password_hash = hash_password(req_user.password).await?;
    let user_id = sqlx::query_scalar!(
        "INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING user_id;",
        req_user.username,
        req_user.email,
        password_hash
    )
    .fetch_one(&state.db)
    .await
    .on_constraint("users_username_key", |_| {
        ApiError::unprocessable_entity([("username", "帳號已存在。")])
    })
    .on_constraint("users_email_key", |_| {
        ApiError::unprocessable_entity([("email", "信箱已存在。")])
    })
    .on_constraint("users_username_check", |_| {
        ApiError::unprocessable_entity([("username", "帳號不可為空。")])
    })
    .on_constraint("users_email_check", |_| {
        ApiError::unprocessable_entity([("email", "信箱不可為空。")])
    })
    .on_constraint("users_password_hash_check", |_| {
        ApiError::unprocessable_entity([("password", "密碼不可為空。")])
    })?;

    let token = AuthUser(user_id).to_jwt(&state.config);
    let user = User {
        email: req_user.email,
        username: req_user.username,
        token,
    };

    Ok(ApiResponse::new_with_message("註冊成功", user))
}

async fn login_user(
    State(state): State<AppState>,
    WithRejection(Json(req_user), _): WithRejection<Json<LoginUser>, ApiError>,
) -> Result<ApiResponse<User>> {
    let user = sqlx::query!(
        r"SELECT user_id, username, email, password_hash FROM users WHERE username = $1;",
        req_user.username
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::custom(
        StatusCode::UNAUTHORIZED,
        "登入失敗",
        "帳號或密碼不正確。",
    ))?;

    verify_password(req_user.password, user.password_hash)
        .await
        .map_err(|_| {
            ApiError::custom(StatusCode::UNAUTHORIZED, "登入失敗", "帳號或密碼不正確。")
        })?;

    let token = AuthUser(user.user_id).to_jwt(&state.config);
    let user = User {
        username: user.username,
        email: user.email,
        token,
    };

    Ok(ApiResponse::new_with_message("登入成功", user))
}

async fn get_current_user(
    AuthUserToken(user_id, token): AuthUserToken,
    State(state): State<AppState>,
) -> Result<ApiResponse<User>> {
    let user = sqlx::query!(
        "SELECT username, email FROM users WHERE user_id = $1",
        user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or(ApiError::Unauthorized)?;

    let user = User {
        username: user.username,
        email: user.email,
        token,
    };

    Ok(ApiResponse::new(user))
}

async fn update_user(
    AuthUserToken(user_id, token): AuthUserToken,
    State(state): State<AppState>,
    WithRejection(Json(req_user), _): WithRejection<Json<UpdateUser>, ApiError>,
) -> Result<ApiResponse<User>> {
    if req_user == Default::default() {
        return get_current_user(AuthUserToken(user_id, token), State(state)).await;
    }

    let password_hash = if let Some(password) = req_user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        r"
        UPDATE users
        SET
            username = COALESCE($1, username),
            email = COALESCE($2, email),
            password_hash = COALESCE($3, password_hash)
        WHERE user_id = $4
        RETURNING username, email
        ",
        req_user.username,
        req_user.email,
        password_hash,
        user_id
    )
    .fetch_one(&state.db)
    .await
    .on_constraint("users_username_key", |_| {
        ApiError::unprocessable_entity([("username", "帳號已存在。")])
    })
    .on_constraint("users_email_key", |_| {
        ApiError::unprocessable_entity([("email", "信箱已存在。")])
    })
    .on_constraint("users_username_check", |_| {
        ApiError::unprocessable_entity([("username", "帳號不可為空。")])
    })
    .on_constraint("users_email_check", |_| {
        ApiError::unprocessable_entity([("email", "信箱不可為空。")])
    })
    .on_constraint("users_password_hash_check", |_| {
        ApiError::unprocessable_entity([("password", "密碼不可為空。")])
    })?;

    let user = User {
        username: user.username,
        email: user.email,
        token,
    };

    Ok(ApiResponse::new_with_message("更新成功", user))
}

async fn hash_password(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
            .to_string())
    })
    .await
    .context("panic in generating password hash")?
}

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let parsed_hash = PasswordHash::new(&password_hash).context("invalid password hash")?;

        Ok(Argon2::default()
            .verify_password(&password.as_bytes(), &parsed_hash)
            .map_err(|_| anyhow::anyhow!("incorrect password"))?)
    })
    .await
    .context("panic in `verify_password`")?
}
