use std::net::SocketAddr;

use ecupool_config::{Config, get_env, load_config};
use eyre::WrapErr;
use tokio::net::TcpListener;
use tracing_panic::panic_hook;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub mod controllers;
pub mod erro;
pub mod middlewares;
pub mod routes;
pub mod state;

pub async fn run() -> eyre::Result<()> {
    let env = get_env().wrap_err("Cannot get environment!")?;
    let config: Config = load_config(env).wrap_err("Cannot load config!")?;

    let addr = SocketAddr::from(([127, 0, 0, 1], config.server.port));

    let app_state = state::init_app_state(config).await;
    tracing::info!("HERE2");
    let app = routes::init_routes(app_state);
    tracing::info!("HERE3");

    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {addr}");
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(filter)
        .init();

    std::panic::set_hook(Box::new(panic_hook));
}
