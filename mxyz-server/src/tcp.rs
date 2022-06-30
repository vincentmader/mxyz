use futures_util::{future, StreamExt, TryStreamExt};
use log::info;
use std::io::Error;
use tokio::net::{TcpListener, TcpStream};

const HOST: &'static str = "127.0.0.1";
const PORT: usize = 1234;

#[tokio::main]
pub async fn start_tcp_listener() -> Result<(), Error> {
    let addr = format!("{}:{}", HOST, PORT);

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

// ============================================================================

use tokio_tungstenite::tungstenite::Message;

async fn accept_connection(stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    info!("Peer address: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    info!("New WebSocket connection: {}", addr);

    let (write, read) = ws_stream.split();
    // We should not forward messages other than text or binary.

    println!("{:?}", read);
    // read.read();

    let contents = read
        .try_filter(|msg| future::ready(msg.is_text() || msg.is_binary()))
        // contents.map(|i| {
        //     let j = i.clone().unwrap();
        //     println!("{}", j);
        // });
        // for i in contents {
        //     let j = i.unwrap();
        //     println!("{}", i);
        // }
        // .map(|i| {
        //     println!("{}", &i.unwrap());
        //     i
        // })
        .map(|i| match &i {
            Ok(e) => {
                // println!("{:?}", e);
                match e {
                    Message::Text(bar) => i,
                    Message::Binary(bar) => {
                        let foo: Vec<u8> = bar.clone();
                        println!("binary message: {:?}", foo);

                        use mxyz_network::package::request::Request;
                        use mxyz_network::package::Package;

                        let foo = Package::from_bytes(foo);

                        match foo {
                            Package::Request(request) => match request {
                                Request::GetUpdatedStates(state_id) => {
                                    mxyz_database::establish_connection();

                                    println!("hurra! {}", state_id);
                                }
                            },
                            Package::Response(response) => {}
                        };

                        let vec: Vec<u8> = vec![147, 255];
                        // TODO
                        // - append state (systems+entities+world) to vec
                        Result::Ok(tokio_tungstenite::tungstenite::Message::Binary(vec))
                    }
                    Message::Ping(bar) => i,
                    Message::Pong(bar) => i,
                    Message::Close(bar) => i,
                    Message::Frame(bar) => i,
                }
            }
            Err(e) => {
                println!("{:?}", e);
                i
            }
        })
        .forward(write)
        .await
        .expect("Failed to forward messages");
    contents
}
