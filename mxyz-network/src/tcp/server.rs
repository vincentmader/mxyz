use crate::message::Message as MpscMessage;
use crate::mpsc::MpscSender;
use crate::package::request::Request;
use crate::package::response::Response;
use crate::package::Package;
use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use std::io::Error;
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

/// TCP Server
pub struct TcpServer {
    address: String,
    mpsc_sender: Arc<Mutex<MpscSender>>,
}
impl TcpServer {
    /// Creates new TCP Server instance
    pub fn new(host: &str, port: u16, mpsc_sender: MpscSender) -> Self {
        let address = format!("{}:{}", host, port);
        let mpsc_sender = Arc::new(Mutex::new(mpsc_sender));
        TcpServer {
            address,
            mpsc_sender,
        }
    }

    /// Starts TCP Listener
    pub async fn init(self) -> Result<(), Error> {
        let try_socket = TcpListener::bind(&self.address).await;
        let listener = try_socket.expect("Failed to bind");
        info!("Listening on: {}", self.address);

        let mpsc_sender = Arc::clone(&self.mpsc_sender);
        while let Ok((stream, _)) = listener.accept().await {
            // let mpsc_sender = mpsc_sender.lock().unwrap().take();
            // tokio::spawn(accept_connection(stream, mpsc_sender));
            accept_connection(stream, &mpsc_sender).await;

            // tokio::spawn(accept_connection(stream));
        }
        Ok(())
    }
}

// ============================================================================

// async fn accept_connection(stream: TcpStream) {
// async fn accept_connection(stream: TcpStream, tx: MutexGuard<'_, mpsc::Sender<MpscMessage>>) {
// async fn accept_connection(stream: TcpStream, mpsc_sender: MpscSender) {
async fn accept_connection(stream: TcpStream, mpsc_sender: &Arc<Mutex<MpscSender>>) {
    // Gets address.
    let address = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", address);

    // Accepts asynchronous web-socket tcp byte-stream.
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("New WebSocket connection: {}", address);

    // Splits stream into read & write components.
    let (write, read) = ws_stream.split();

    // Loops over incoming messages & maps onto responses.
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .map(|i| handle_message(i))
        .forward(write)
        .await
        .expect("Failed to forward messages");

    // NOTE should not forward messages other than text or binary
}

// ============================================================================

type MessageResult = Result<Message, tokio_tungstenite::tungstenite::Error>;

pub fn handle_message(msg: MessageResult) -> MessageResult {
    match &msg {
        Ok(e) => match e {
            Message::Binary(bytes) => handle_binary_message(bytes.to_vec()),
            // TODO implement below
            Message::Text(_) => msg,
            Message::Ping(_) => Ok(Message::Pong(vec![7, 4, 1])),
            Message::Pong(_) => Ok(Message::Ping(vec![1, 4, 7])),
            Message::Close(_) => msg,
            Message::Frame(_) => msg,
        },
        Err(e) => {
            println!("{:?}", e);
            msg
        }
    }
}

pub fn handle_binary_message(bytes: Vec<u8>) -> MessageResult {
    let bytes: Vec<u8> = bytes.clone();
    // println!("incoming binary message: {:?}", bytes);
    let package = Package::from_bytes(bytes);
    // println!("incoming package: {:?}", package);

    let response = match package {
        Package::Request(request) => handle_request(request),
        Package::Response(response) => handle_response(response),
    };
    // Convert package to bytes and return.
    let bytes = response.to_bytes();
    Ok(Message::Binary(bytes))
}

pub fn handle_request(request: Request) -> Package {
    match request {
        Request::GetUpdatedStates(last_update) => {
            println!("Incoming: get updated states (since state {})", last_update);
            // Load states from database.
            let states = vec![];
            // let states = mxyz_engine::Engine::get_updated_states(last_update);
            println!("Loaded {} states from database!", states.len());
            // Return state-vector response
            let response = Response::StateVector(states);
            Package::Response(response)
        }
        Request::AddEngine => {
            let response = Response::AddedEngine;
            Package::Response(response)
        }
    }
}

pub fn handle_response(response: Response) -> Package {
    match response {
        Response::Empty => Package::Response(Response::Empty),
        // TODO
        Response::StateVector(_) => Package::Response(Response::Empty),
        // TODO
        Response::AddedEngine => Package::Response(Response::Empty),
    }
}
