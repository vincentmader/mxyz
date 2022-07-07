use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use mxyz_database::models::state::StateQuery;
use mxyz_network::package::command::Command;
use mxyz_network::package::request::Request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use std::io::Error;
use std::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;
// use mxyz_universe::preset::SimulationVariant;
// use mxyz_engine::state::State;

/// TCP Server
pub struct TcpServer {
    address: String,
    tx: mpsc::Sender<Package>,
}
impl TcpServer {
    /// Creates new TCP Server instance
    pub fn new(host: &str, port: u16, tx: mpsc::Sender<Package>) -> Self {
        let address = format!("{}:{}", host, port);
        TcpServer { address, tx }
    }

    /// Starts TCP Listener
    pub async fn init(self) -> Result<(), Error> {
        let try_socket = TcpListener::bind(&self.address).await;
        let listener = try_socket.expect("Failed to bind");
        info!("Listening on: {}", self.address);

        while let Ok((stream, _)) = listener.accept().await {
            let tx = self.tx.clone();
            tokio::spawn(accept_connection(stream, tx));
        }
        Ok(())
    }
}

// ============================================================================

async fn accept_connection(stream: TcpStream, tx: mpsc::Sender<Package>) {
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
    // - should not forward messages other than text or binary
    read.try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        .map(move |i| handle_message(i, &tx))
        .forward(write)
        .await
        .expect("Failed to forward messages");
}

// ============================================================================

type MessageResult = Result<Message, tokio_tungstenite::tungstenite::Error>;

pub fn handle_message(msg: MessageResult, tx: &mpsc::Sender<Package>) -> MessageResult {
    match &msg {
        Ok(e) => match e {
            Message::Binary(bytes) => handle_binary_message(bytes.to_vec(), tx),
            _ => todo!("handle non-binary messages")
            // Message::Text(_) => msg,
            // Message::Ping(_) => Ok(Message::Pong(vec![7, 4, 1])),
            // Message::Pong(_) => Ok(Message::Ping(vec![1, 4, 7])),
            // Message::Close(_) => msg,
            // Message::Frame(_) => msg,
        },
        Err(e) => {
            println!("{:?}", e);
            msg
        }
    }
}

pub fn handle_binary_message(bytes: Vec<u8>, tx: &mpsc::Sender<Package>) -> MessageResult {
    let bytes: Vec<u8> = bytes.clone();
    // println!("incoming binary message: {:?}", bytes);
    let package = Package::from_bytes(bytes);
    // println!("incoming package: {:?}", package);

    let response = match package {
        Package::Request(request) => handle_request(request, &tx),
        Package::Response(response) => handle_response(response),
        Package::Command(command) => handle_command(command),
    };
    // Convert package to bytes and return.
    let bytes = response.to_bytes();
    Ok(Message::Binary(bytes))
}

pub fn handle_request(request: Request, tx: &mpsc::Sender<Package>) -> Package {
    match request {
        Request::AddClient => {
            let db_conn = mxyz_database::establish_connection();
            let client = mxyz_database::models::client::create_client(&db_conn);
            let client_id = client.client_id as usize;
            let response = Response::AddedClient(client_id);
            Package::Response(response)
        }

        Request::AddEngine(client_id, simulation_variant) => {
            // let db_conn = mxyz_database::establish_connection();
            // let engine = mxyz_database::models::engine::create_engine(&db_conn, client_id);
            // let engine_id = engine.engine_id as usize;

            // TODO do differently
            // - get engine-id from channel somehow (2nd channel? request-response instead?)
            // - or: see above (add to db from here, only start engine through mpsc)
            let engine_id = mxyz_database::models::engine::get_db_engines().len() + 1;

            let request = Request::AddEngine(client_id, simulation_variant);
            let package = Package::Request(request);
            tx.send(package).unwrap(); // TODO do this differently, it's just forwarding the msg

            let response = Response::AddedEngine(engine_id);
            Package::Response(response)
        }

        Request::RemoveEngine(_engine_id) => todo!("remove engine"),

        Request::GetUpdatedStates(engine_id, last_sync_id) => {
            let state_query = StateQuery::Since(last_sync_id as i32);
            println!("Incoming: get-updated-states: {:?}", state_query);
            let states = mxyz_database::models::state::get_states(engine_id as i32, &state_query);
            println!("Loaded {} states from database!", states.len());

            // Return state-vector response
            let response = Response::StateVector(engine_id, states);
            Package::Response(response)
        }
    }
}

pub fn handle_response(response: Response) -> Package {
    match response {
        _ => todo!("handle responses"),
        // Response::Empty => Package::Response(Response::Empty),
        // Response::StateVector(_) => Package::Response(Response::Empty),
        // Response::AddedEngine => Package::Response(Response::Empty),
        // Response::AddedClient(client_id) => Package::Response(Response::Empty),
    }
}

pub fn handle_command(command: Command) -> Package {
    match command {
        // Command::SaveStatesToDatabase => Package::Response(Response::Empty),
        _ => todo!("handle commands"),
    }
}
