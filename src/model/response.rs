//! 响应模型模块
//!
//! 定义统一的API响应格式和错误处理

#![allow(unused)]

use axum::body::Body;
use axum::http::{HeaderValue, Response, StatusCode, header};
use axum::response::IntoResponse;
use serde::Serialize;
use serde_repr::*;
use std::fmt::{Debug, Display};
use tracing::{debug, error};

/// 统一API响应结构
///
/// # 泛型参数
/// - `T`: 数据负载类型
/// - `W`: 消息类型(默认为String)
///
/// # 字段
/// - `code`: 响应状态码
/// - `data`: 响应数据(可选)
/// - `message`: 响应消息(可选)
///
/// # 序列化规则
/// 使用`skip_serializing_if`跳过空字段
#[derive(Debug, Default, Serialize)]
pub struct Resp<T, W = String> {
    pub code: Code,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<W>,
}

/// 实现IntoResponse trait，使Resp可以直接作为axum响应
impl<T> IntoResponse for Resp<T>
where
    T: Serialize,
{
    /// 将Resp转换为HTTP响应
    ///
    /// # 处理流程
    /// 1. 序列化为JSON字符串
    /// 2. 失败时返回错误响应
    /// 3. 设置Content-Type为application/json
    /// 4. 记录调试日志
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

/// 响应状态码枚举
///
/// 使用u16作为底层表示，定义标准化的响应状态码
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Default)]
#[repr(u16)]
pub enum Code {
    /// 成功(默认值)
    #[default]
    Success = 0,
    /// 响应序列化失败
    RespSerializeFailed = 100,
    /// 内部服务器错误
    InternalError = 500,
}
