use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use mxyz_network::package::request::Request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use std::io::Error;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

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
    println!("incoming binary message: {:?}", bytes);
    let package = Package::from_bytes(bytes);
    // println!("incoming package: {:?}", package);

    match package {
        Package::Request(request) => handle_request(request),
        Package::Response(response) => handle_response(response),
    }
}

pub fn handle_request(request: Request) -> MessageResult {
    let package = match request {
        Request::GetUpdatedStates(last_update) => {
            println!("Incoming: get updated states (since state {})", last_update);
            // Load states from database.
            let states = mxyz_engine::Engine::get_updated_states(last_update);
            println!("Loaded {} states from database!", states.len());
            // Return state-vector response
            let response = Response::StateVector(states);
            Package::Response(response)
        }
    };
    // Convert package to bytes and return.
    let bytes = package.to_bytes();
    Ok(Message::Binary(bytes))
}

pub fn handle_response(response: Response) -> MessageResult {
    match response {
        Response::StateVector(_) => {}
    }
    // Convert package to bytes and return.
    let bytes = vec![]; // TODO
    Ok(Message::Binary(bytes))
}
