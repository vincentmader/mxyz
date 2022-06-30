use std::io::Error;
pub mod server;
use server::TcpServer;

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 1234;

#[tokio::main]
pub async fn start_tcp_listener() -> Result<(), Error> {
    let tcp_server = TcpServer::new(HOST, PORT);
    tcp_server.start_tcp_listener().await.unwrap();
    Ok(())
}
