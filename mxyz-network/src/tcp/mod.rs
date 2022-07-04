use std::io::Error;
pub mod server;
use crate::message::Message;
use crate::mpsc::MpscSender;
use server::TcpServer;
use std::sync::mpsc;

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 1234;

#[tokio::main]
pub async fn start_tcp_listener(mpsc_sender: MpscSender) -> Result<(), Error> {
    let tcp_server = TcpServer::new(HOST, PORT, mpsc_sender);
    tcp_server.init().await.unwrap();
    Ok(())
}
