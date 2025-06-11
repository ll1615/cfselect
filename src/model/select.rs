//! 选择状态模型
//!
//! 定义IP选择任务的状态枚举

use serde::Serialize;

/// IP选择任务状态
#[derive(Debug, Clone, Serialize)]
pub enum Status {
    /// 等待中(初始状态)
    Pending,
    /// 处理中(任务进行中)
    Processing,
    /// 成功(任务完成)
    Success,
    /// 失败(包含错误信息)
    Failed(String),
}
