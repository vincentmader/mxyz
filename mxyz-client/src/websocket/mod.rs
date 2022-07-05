use wasm_bindgen::prelude::*;
pub mod client;
// use mxyz_network::package::Package;
// use std::sync::mpsc::Receiver;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 1234;

// #[wasm_bindgen]
// pub fn start_client(rx: Receiver<Package>) -> Result<(), JsValue> {
pub fn start_client() -> Result<(), JsValue> {
    let mut client = client::WebSocketClient::new(HOST, PORT);
    client.init()?;
    Ok(())
}
