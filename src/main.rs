#[tokio::main(flavor = "multi_thread", worker_threads = 100)]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_windows::init();
    wei_env::bin_init("wei-server");
    use single_instance::SingleInstance;
    let instance = wei_single::SingleInstance::new("wei-server").unwrap();
    if !instance.is_single() { 
        std::process::exit(1);
    };

    wei_server::start().await?;
    
    Ok(())
}
