use axum::Router;
use axum::routing::get;

mod routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei-server");
    let instance = single_instance::SingleInstance::new("wei-server")?;
    if !instance.is_single() { 
        std::process::exit(1);
    };

    let mut port = 1115;

    // 循环查找可用端口
    while !is_port_available(port) {
        port += 1;
    }

    // 把端口使用 serde_toml 写入数据到 .wei/server.dat
    let file_server = wei_env::home_dir()? + "server.dat";
    let mut server = std::fs::File::create(file_server)?;
    let data = format!("{}", port);
    use std::io::Write;
    server.write_all(&data.into_bytes())?;

    // axum 启动之后，不阻塞进程
    let handle = tokio::spawn(async move {
        // 构建我们的路由表
        let app = Router::new()
            .route("/index", get(routes::index::index))
            .route("/image", get(routes::image::index))
            .route("/model", get(routes::model::index))
            .route("/", get(|| async { "wei-server" }));

        // 绑定port端口
        let address = format!("127.0.0.1:{}", port);
        println!("Server running on {}", address);
        axum::Server::bind(&address.parse().unwrap())
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    loop {
        if wei_env::status() == "0" {
            // 当程序接受到退出信号时，关闭 axum 服务
            handle.abort();
            return Ok(());
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }

}

fn is_port_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}