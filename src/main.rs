// use wei_single::SingleInstance;
use regex::Regex;

mod routes;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = 1115;

    // 循环查找可用端口
    // while !is_port_available(port) {
    //     port += 1;
    // }

    let output = std::process::Command::new("ifconfig")
    .arg("eth0")
    .output()?;

    let output = String::from_utf8_lossy(&output.stdout);

    let re = Regex::new(r"inet (\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})")?;
    let cap = re.captures(&output).ok_or("")?;
    let ip = cap[1].to_string();

    if ip == "" {
        println!("error: can not get ip address");
        std::process::exit(0);
    }

    let file_server = "./wsl.dat";
    let mut server = std::fs::File::create(file_server)?;
    let data = format!("http://{}:{}", ip, port);
    use std::io::Write;
    server.write_all(&data.into_bytes())?;

    // 构建我们的路由表
    let app = routes::routes();

    // 绑定port端口
    let address = format!("0.0.0.0:{}", port);

    println!("Server running on {}", address);
    axum::Server::bind(&address.parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())

}

fn _is_port_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("0.0.0.0", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}