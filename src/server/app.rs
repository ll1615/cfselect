//! 应用主逻辑模块
//!
//! 负责:
//! - 应用启动和初始化
//! - 路由设置
//! - 优雅关闭处理

use crate::configure::AppConfig;
use crate::router;
use crate::server::state::AppState;
use tokio::signal;
use tracing::info;

/// 应用主入口函数
///
/// # 返回值
/// - 成功: Ok(())
/// - 失败: 返回anyhow::Error
pub async fn run() -> anyhow::Result<()> {
    // 1. 读取应用配置
    let conf = AppConfig::read()?;

    // 2. 初始化日志系统
    // _guard确保日志系统在整个应用生命周期保持活动
    let _guard = conf.init_tracing()?;

    // 3. 创建共享应用状态
    let state = AppState::new(conf.clone()).await?;

    // 4. 配置路由
    let app = router::setup(state);

    // 5. 绑定监听地址
    let listener = tokio::net::TcpListener::bind(conf.listen.get_socket_addr()?).await?;
    info!("🚀 listening on {}", &listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// 优雅关闭信号处理器
///
/// # 支持的信号类型
/// - Ctrl+C (所有平台)
/// - SIGTERM (Unix系统)
pub async fn shutdown_signal() {
    // 处理Ctrl+C信号
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // Unix系统特有的终止信号处理
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    // 非Unix系统使用空等待
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    // 使用tokio::select!等待任意信号
    tokio::select! {
        _ = ctrl_c => {
            println!("Ctrl+C signal received.");
        },
        _ = terminate => {
            println!("Terminate signal received.");
        },
        else => (),
    }
}
