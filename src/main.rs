mod api;
mod client;
mod configure;
mod model;
mod router;
mod server;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    server::app::run().await
}
