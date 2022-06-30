use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use mxyz_network::package::request::Request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use std::io::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

const HOST: &'static str = "127.0.0.1";
const PORT: u16 = 1234;

#[tokio::main]
pub async fn start_tcp_listener() -> Result<(), Error> {
    let tcp_server = TcpServer::new(HOST, PORT);
    tcp_server.start_tcp_listener().await.unwrap();
    Ok(())
}

// ============================================================================

pub struct TcpServer {
    address: String,
}
impl TcpServer {
    pub fn new(host: &str, port: u16) -> Self {
        let address = format!("{}:{}", host, port);
        TcpServer { address }
    }
    pub async fn start_tcp_listener(self) -> Result<(), Error> {
        let try_socket = TcpListener::bind(&self.address).await;
        let listener = try_socket.expect("Failed to bind");
        info!("Listening on: {}", self.address);

        while let Ok((stream, _)) = listener.accept().await {
            tokio::spawn(accept_connection(stream));
        }
        Ok(())
    }
}

// ============================================================================

async fn accept_connection(stream: TcpStream) {
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
    let package = Package::from_bytes(bytes);
    // println!("incoming binary message: {:?}", bytes);
    // println!("incoming package: {:?}", package);

    match package {
        Package::Request(request) => handle_request(request),
        Package::Response(response) => handle_response(response),
    }

    // let vec: Vec<u8> = vec![147, 255];
    // Result::Ok(tokio_tungstenite::tungstenite::Message::Binary(vec))
}

pub fn handle_request(request: Request) -> MessageResult {
    let package = match request {
        Request::GetUpdatedStates(state_id) => {
            mxyz_database::establish_connection();
            println!("hurra! {}", state_id);

            let last_update = 0;
            let states = mxyz_engine::Engine::get_updated_states(last_update);
            println!("{:?}", states);

            let states = vec![]; // TODO
            let response = Response::StateVector(states);
            Package::Response(response)
        }
    };
    let bytes = package.to_bytes();
    Ok(Message::Binary(bytes))
}

pub fn handle_response(response: Response) -> MessageResult {
    match response {
        Empty => {}
        StateVector => {}
    }
    let bytes = vec![]; // TODO
    Ok(Message::Binary(bytes))
}
