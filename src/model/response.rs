#![allow(unused)]

use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;
use serde::Serialize;
use serde_repr::*;
use std::fmt::{Debug, Display};
use tracing::{debug, error};

#[derive(Debug, Default, Serialize)]
pub struct Resp<T, W = String> {
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<W>,
}

impl<T> IntoResponse for Resp<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let json = serde_json::to_string(&self).unwrap_or_else(|e| {
            error!("Failed to serialize response: {}", e);
            serde_json::json!({
                "code": Code::RespSerializeFailed,
                "data": null,
                "message": "Internal Server Error"
            })
            .to_string()
        });
        debug!("Response: {}", json);

        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
            )
            .body(Body::from(json))
            .unwrap_or_default()
    }
}

impl<E: Display + Debug> From<E> for Resp<()> {
    fn from(err: E) -> Self {
        Resp {
            code: Code::InternalError,
            data: None,
            message: Some(err.to_string()),
        }
    }
}

pub fn success<T: Serialize>() -> Resp<T> {
    Resp {
        code: Code::Success,
        data: None,
        message: None,
    }
}

pub fn success_data<T: Serialize>(data: T) -> Resp<T> {
    Resp {
        code: Code::Success,
        data: Some(data),
        message: None,
    }
}

pub fn fail<T: Serialize>(code: Code, message: String) -> Resp<T> {
    Resp {
        code,
        data: None,
        message: Some(message),
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u16)]
pub enum Code {
    #[default]
    Success = 0,
    RespSerializeFailed = 100,
    InternalError = 500,
}
