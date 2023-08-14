use std::env;
use axum::Router;
use axum::routing::get;

#[tokio::main]
async fn main() {
    wei_env::write(&wei_env::dir_bin(), "wei-server", &env::current_exe().unwrap().display().to_string()).unwrap();

    // 构建我们的路由表
    let app = Router::new().route("/", get(|| async { "Hello, axum!" }));

    // 运行应用程序
    axum::Server::bind(&"0.0.0.0:1115".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
