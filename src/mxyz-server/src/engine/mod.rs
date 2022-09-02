pub mod engine_runner_v2;
use engine_runner_v2::EngineRunner;
use mxyz_network::mpsc_msg::MpscMessage;
use std::io::Error;
use std::sync::mpsc;

#[tokio::main]
pub async fn start_engine_runner(rx: mpsc::Receiver<MpscMessage>) -> Result<(), Error> {
    std::thread::spawn(move || {
        let mut engine_runner = EngineRunner::new(rx);
        engine_runner.init();
    });
    Ok(())
}
