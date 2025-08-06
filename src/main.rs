mod route;
mod handle;
mod mystruct;
use crate::route::creatroute;

use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();


    tracing::debug!("开始初始化...");
    let addr = SocketAddr::from(([0,0,0,0],3000));
    tracing::info!("服务器监听于 {}", addr);
    let http_listener=tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        http_listener,
        creatroute().into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap()
}