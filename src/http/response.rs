use std::{borrow::Cow, collections::HashMap};

use axum::{extract::rejection::JsonRejection, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use serde_json::json;

pub struct ApiResponse<T> {
    message: Cow<'static, str>,
    data: T,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),

    #[error("認證失敗")]
    Unauthorized,

    #[error("資料庫錯誤")]
    SQLx(#[from] sqlx::Error),

    #[error("資料有誤")]
    UnprocessableEntity(HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>),

    #[error("JSON 格式錯誤")]
    JsonExtractorRejection(#[from] JsonRejection),

    #[error("Custom Error")]
    Custom {
        status: StatusCode,
        title: Cow<'static, str>,
        message: Cow<'static, str>,
    },
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    title: Cow<'static, str>,
    message: Cow<'static, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<Vec<ErrorDetail>>,
}

#[derive(Debug, Serialize)]
struct ErrorDetail {
    name: Cow<'static, str>,
    messages: Vec<Cow<'static, str>>,
}

// ===== ApiResponse =====

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        ApiResponse {
            message: "OK".into(),
            data,
        }
    }

    pub fn new_with_message(message: impl Into<Cow<'static, str>>, data: T) -> Self {
        ApiResponse {
            message: message.into(),
            data,
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(json!({
            "message": self.message,
            "data": self.data
        }))
        .into_response()
    }
}

// ===== ApiError =====

impl ApiError {
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut map = HashMap::new();

        for (k, v) in errors {
            map.entry(k.into()).or_insert_with(Vec::new).push(v.into());
        }

        ApiError::UnprocessableEntity(map)
    }

    pub fn custom<T>(status: StatusCode, title: T, message: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        ApiError::Custom {
            status,
            title: title.into(),
            message: message.into(),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Anyhow(_) | Self::SQLx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::JsonExtractorRejection(rejection) => rejection.status(),
            Self::Custom { status, .. } => status.clone(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let status_code = self.status_code();
        let message = self.to_string().into();
        let response = match self {
            ApiError::Anyhow(e) => {
                let shows_message = e.source().is_some();
                ErrorResponse {
                    title: message,
                    message: if shows_message {
                        e.root_cause().to_string().into()
                    } else {
                        "".into()
                    },
                    details: None,
                }
            }
            ApiError::SQLx(e) => ErrorResponse {
                title: message,
                message: e.to_string().into(),
                details: None,
            },
            ApiError::Unauthorized => ErrorResponse {
                title: message,
                message: "".into(),
                details: None,
            },
            ApiError::UnprocessableEntity(errors) => ErrorResponse {
                title: message,
                message: "".into(),
                details: if errors.is_empty() {
                    None
                } else {
                    Some(
                        errors
                            .into_iter()
                            .map(|(k, v)| ErrorDetail {
                                name: k,
                                messages: v,
                            })
                            .collect(),
                    )
                },
            },
            ApiError::JsonExtractorRejection(rejection) => ErrorResponse {
                title: message,
                message: rejection.body_text().into(),
                details: None,
            },
            Self::Custom { title, message, .. } => ErrorResponse {
                title,
                message,
                details: None,
            },
        };

        (status_code, Json(response)).into_response()
    }
}
