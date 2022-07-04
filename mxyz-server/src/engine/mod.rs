use mxyz_network::package::Package;
use runner::EngineRunner;
use std::io::Error;
use std::sync::mpsc;
pub mod runner;

#[tokio::main]
pub async fn start_engine_runner(
    tx: mpsc::Sender<Package>,
    rx: mpsc::Receiver<Package>,
) -> Result<(), Error> {
    std::thread::spawn(move || {
        let mut engine_runner = EngineRunner::new(tx, rx);
        engine_runner.init();
    });
    Ok(())
}
