use crate::http::response::ApiError;

pub trait ResultExt<T> {
    fn on_constraint<F>(self, name: &str, map_err: F) -> Result<T, ApiError>
    where
        F: FnOnce(Box<dyn sqlx::error::DatabaseError>) -> ApiError;
}

impl<T, E> ResultExt<T> for Result<T, E>
where
    E: Into<ApiError>,
{
    fn on_constraint<F>(self, name: &str, map_err: F) -> Result<T, ApiError>
    where
        F: FnOnce(Box<dyn sqlx::error::DatabaseError>) -> ApiError,
    {
        self.map_err(|e| match e.into() {
            ApiError::SQLx(sqlx::Error::Database(e)) if e.constraint() == Some(name) => map_err(e),
            e => e,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::ApiError;

    #[test]
    fn test_fields_error() {
        let e = ApiError::unprocessable_entity([("f1", "v1"), ("f1", "v2"), ("f2", "v1")]);
        match e {
            ApiError::UnprocessableEntity(errors) => {
                assert_eq!(
                    errors,
                    HashMap::from([
                        ("f1".into(), vec!["v1".into(), "v2".into()]),
                        ("f2".into(), vec!["v1".into()])
                    ])
                )
            }
            _ => panic!(),
        }
    }
}
