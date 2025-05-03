mod server;
mod configure;
mod router;
mod api;
mod model;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> anyhow::Result<()> {
    let err = server::app::run().await?;
    Ok(())
}

