#[macro_use]
extern crate wei_log;

mod routes;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_env::bin_init("wei-server");
    use single_instance::SingleInstance;
    let instance = SingleInstance::new("wei-server").unwrap();
    if !instance.is_single() { 
        std::process::exit(1);
    };

    let port = 1115;
    let app = routes::routes();

    let address = format!("127.0.0.1:{}", port);
    println!("Server running on {}", address);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
    
    Ok(())
}
