use std::io::Error;
pub mod server;
use mxyz_network::mpsc_msg::MpscMessage;
use server::TcpServer;
use std::sync::mpsc;

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 1234;

#[tokio::main]
pub async fn start_tcp_listener(tx: mpsc::Sender<MpscMessage>) -> Result<(), Error> {
    let tcp_server = TcpServer::new(HOST, PORT, tx);
    tcp_server.init().await.unwrap();
    Ok(())
}
