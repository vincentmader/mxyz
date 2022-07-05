use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use mxyz_database::models::state::StateQuery;
use mxyz_engine::state::State;
use mxyz_network::package::command::Command;
use mxyz_network::package::request::Request;
use mxyz_network::package::response::Response;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
use std::io::Error;
use std::sync::mpsc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::Message;

/// TCP Server
pub struct TcpServer {
    address: String,
    rx: mpsc::Receiver<Package>,
    tx: mpsc::Sender<Package>,
}
impl TcpServer {
    /// Creates new TCP Server instance
    pub fn new(
        host: &str,
        port: u16,
        tx: mpsc::Sender<Package>,
        rx: mpsc::Receiver<Package>,
    ) -> Self {
        let address = format!("{}:{}", host, port);
        TcpServer { address, rx, tx }
    }

    /// Starts TCP Listener
    pub async fn init(self) -> Result<(), Error> {
        let try_socket = TcpListener::bind(&self.address).await;
        let listener = try_socket.expect("Failed to bind");
        info!("Listening on: {}", self.address);

        while let Ok((stream, _)) = listener.accept().await {
            // let (tx, rx) = mpsc::channel::<Package>();
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
        Request::GetUpdatedStates(last_update) => {
            println!("Incoming: get updated states (since state {})", last_update);
            // Load states from database.
            // - TODO
            let engine_id = 0; // TODO
            let last_sync = 0; //TODO
            let state_query = StateQuery::Since(last_sync);

            let nr_of_steps = 10;
            while mxyz_database::models::state::get_states(engine_id, &state_query).len()
                < nr_of_steps
            {}

            let states = mxyz_database::models::state::get_states(engine_id, &state_query);
            // let states = mxyz_engine::Engine::get_updated_states(last_update);
            println!("Loaded {} states from database!", states.len());
            // Return state-vector response
            let response = Response::StateVector(states);
            Package::Response(response)
        }
        Request::AddEngine(client_id, simulation_variant) => {
            // let tx = tx.clone();
            // let simulation_variant = SimulationVariant::ThreeBodyMoon;
            tx.send(Package::Request(Request::AddEngine(
                client_id,
                simulation_variant,
            )))
            .unwrap(); // TODO do this differently, it's just forwarding the msg

            // tx.send(Package::Request(request.clone())).unwrap();
            // tx.send(Package::Request(request)).unwrap();

            let response = Response::AddedEngine;
            Package::Response(response)
        }
        Request::RemoveEngine(engine_id) => {
            let response = Response::Empty;
            Package::Response(response)
        }
        Request::AddClient => {
            let client_id = mxyz_database::models::client::get_db_clients().len();
            // let client_id = std::cmp::max(0, client_id); // TODO needed?
            let db_conn = mxyz_database::establish_connection();
            mxyz_database::models::client::create_client(&db_conn, client_id);
            let response = Response::AddedClient(client_id);
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
        // TODO
        Response::AddedClient(client_id) => Package::Response(Response::Empty),
    }
}

pub fn handle_command(command: Command) -> Package {
    match command {
        Command::SaveStatesToDatabase => Package::Response(Response::Empty),
    }
}
