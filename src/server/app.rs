use crate::configure::AppConfig;
use crate::router;
use crate::server::state::AppState;
use tokio::signal;
use tracing::info;

pub async fn run() -> anyhow::Result<()> {
    let conf = AppConfig::read()?;
    let _guard = conf.init_tracing()?;

    let state = AppState::new(conf.clone()).await?;
    let app = router::setup(state);

    let listener = tokio::net::TcpListener::bind(conf.listen.get_socket_addr()?).await?;
    info!("🚀 listening on {}", &listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal().await)
        .await?;

    Ok(())
}

pub async fn shutdown_signal() -> impl Future<Output = ()> {
    async {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

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
}
