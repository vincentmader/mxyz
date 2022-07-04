use std::io::Error;
pub mod server;
use mxyz_network::package::Package;
use server::TcpServer;
use std::sync::mpsc;

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 1234;

#[tokio::main]
pub async fn start_tcp_listener(
    tx: mpsc::Sender<Package>,
    rx: mpsc::Receiver<Package>,
) -> Result<(), Error> {
    let tcp_server = TcpServer::new(HOST, PORT, tx, rx);
    tcp_server.init().await.unwrap();
    Ok(())
}
