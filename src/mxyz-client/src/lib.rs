// pub use wasm_bindgen_rayon::init_thread_pool;
pub mod client;
pub mod config;
pub mod renderer;
pub mod utils;
pub mod websocket;
pub use client::Client;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn init_client(runner_variant: usize, category: String, simulation_variant: String) {
    let mut client = Client::new(runner_variant);
    client.init(&category, &simulation_variant).await;
}
