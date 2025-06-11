mod api;
mod client;
mod configure;
mod model;
mod router;
mod server;

/// 应用主入口
#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    // 启动服务器应用
    server::app::run().await
}
