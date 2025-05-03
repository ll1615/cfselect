use axum::body::Body;
use axum::http::{header, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use serde_repr::*;
use tracing::{debug, error};

#[derive(Debug, Default, Serialize)]
pub struct Resp<T> {
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
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
                }).to_string()
        });
        debug!("Response: {}", json);

        Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()))
            .body(Body::from(json))
            .unwrap_or_default()
    }
}

pub fn success<T: Serialize>(data: Option<T>) -> Json<Resp<T>> {
    Json(Resp {
        code: Code::Success,
        data,
        message: None,
    })
}

pub fn success2<T: Serialize>(data: Option<T>) -> Resp<T> {
    Resp {
        code: Code::Success,
        data,
        message: None,
    }
}

pub fn fail<T: Serialize>(code: Code, message: String) -> Json<Resp<T>> {
    Json(Resp {
        code,
        data: None,
        message: Some(message),
    })
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u16)]
pub enum Code {
    #[default]
    Success = 0,
    RespSerializeFailed = 100,
}