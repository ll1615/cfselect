use crate::configure::AppConfig;
use crate::router;
use crate::server::state::AppState;

pub async fn run() -> anyhow::Result<()> {
    let conf = AppConfig::read()?;
    let _guard = conf.init_tracing()?;

    let state = AppState::new(conf.clone()).await?;
    let app = router::setup(state);

    let listener = tokio::net::TcpListener::bind(conf.listen.get_socket_addr()?).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

