use salvo::prelude::*;

mod handlers;
mod routers;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = routers::init_router();

    println!("🚀 服务启动：http://127.0.0.1:5800");

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}