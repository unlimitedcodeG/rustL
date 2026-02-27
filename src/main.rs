use salvo::prelude::*;

mod routes;


#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::fmt().init();

    let router = routes::init_router();


    print!("🚀 服务启动：http://127.0.0.1:5800");


    let acceptor  =TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}