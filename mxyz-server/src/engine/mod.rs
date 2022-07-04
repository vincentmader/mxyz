use std::io::Error;
use std::sync::mpsc;
pub mod runner;
use mxyz_network::message::Message;
use runner::EngineRunner;

#[tokio::main]
pub async fn start_engine_runner(rx: mpsc::Receiver<Message>) -> Result<(), Error> {
    // Server-Engine Communication: Create Engine-Runner w/ MPSC streaming channel.
    std::thread::spawn(move || {
        let mut engine_runner = EngineRunner::new(rx);
        engine_runner.init();
    });
    Ok(())
}
