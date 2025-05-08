use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Status {
    // 等待中
    Pending,
    // 处理中
    Processing,
    // 成功
    Success,
    // 失败
    Failed(String),
}
