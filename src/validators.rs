use std::collections::HashMap;

use axum::{
    Json,
    extract::{FromRequest, Request, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

use crate::error_handler::AppError;

pub fn format_validation_errors(err: ValidationErrors) -> Value {
    let mut field_errors = serde_json::Map::new();

    for (field, kind) in err.errors() {
        match kind {
            ValidationErrorsKind::Field(errors) => {
                if let Some(error) = errors.first() {
                    let message = error
                        .message
                        .clone()
                        .unwrap_or_else(|| "invalid value".into())
                        .to_string();
                    field_errors.insert(field.to_string(), json!(message));
                }
            }
            ValidationErrorsKind::List(list) => {}
            ValidationErrorsKind::Struct(s) => {}
        }
    }
    Value::Object(field_errors)
}

// #[derive(Debug, Clone, Copy, Default)]
// pub struct ValidatedForm<T>(pub T);

// impl<T, S> FromRequest<S> for ValidatedForm<T>
// where
//     T: DeserializeOwned + Validate,
//     S: Send + Sync,
//     Form<T>: FromRequest<S, Rejection = FormRejection>,
// {
//     type Rejection = ServerError;

//     async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
//         let Form(value) = Form::<T>::from_request(req, state).await?;
//         value.validate()?;
//         Ok(ValidatedForm(value))
//     }
// }

#[derive(Debug, Clone, Copy)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}
