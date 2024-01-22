#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_windows::init();
    wei_env::bin_init("wei");
    
    
    Ok(())
}
