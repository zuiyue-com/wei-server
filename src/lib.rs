#[macro_use]
extern crate wei_log;

mod routes;

pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let mut port = 1115;

    // 循环查找可用端口
    while !is_port_available(port) {
        port += 1;
    }

    let file_server = wei_env::home_dir()? + "server.dat";
    let mut server = std::fs::File::create(file_server)?;
    let data = format!("{}", port);
    use std::io::Write;
    server.write_all(&data.into_bytes())?;

    // 构建我们的路由表
    let app = routes::routes();

    // 绑定port端口
    let address = format!("127.0.0.1:{}", port);
    println!("Server running on {}", address);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())

}

fn is_port_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}