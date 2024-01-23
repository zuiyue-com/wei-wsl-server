use wei_single::SingleInstance;

mod routes;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    wei_windows::init();
    let instance = SingleInstance::new("wei-wsl-server")?;
    if !instance.is_single() { 
        std::process::exit(0);
    };

    let mut port = 51115;

    // 循环查找可用端口
    while !is_port_available(port) {
        port += 1;
    }

    let file_server = "./wsl.dat";
    let mut server = std::fs::File::create(file_server)?;
    let data = format!("{}", port);
    use std::io::Write;
    server.write_all(&data.into_bytes())?;

    // 构建我们的路由表
    let app = routes::routes();

    // 绑定port端口
    let address = format!("[::]:{}", port);

    println!("Server running on {}", address);
    axum::Server::bind(&address.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())

}

fn is_port_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("::", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}