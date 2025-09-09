mod handle;
mod mystruct;
mod route;
use crate::route::creatroute;
use std::net::SocketAddr;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_ansi(false)
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("服务器监听于 {}", addr);
    tracing::info!("/verify/submit_ticket");
    tracing::info!("/verify/submit_phone_number");
    tracing::info!("/verify/submit_auth");
    tracing::info!("Github：https://github.com/Linxiaoyaa/QQVerifyLogin");
    tracing::info!("作者：Linxiaoyaa");
    tracing::info!("交流群：957874828");
    tracing::info!("感谢你的使用，如果帮助到了你，请star一下吧~");
    let http_listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        http_listener,
        creatroute().into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
