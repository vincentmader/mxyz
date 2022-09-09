use mxyz_engine::config::simulation_variant::SimulationVariant;
use mxyz_engine::Engine;
use mxyz_engine_v2::SimulationEngineV2;
use mxyz_network::mpsc_msg;
use mxyz_network::mpsc_msg::MpscMessage;
use std::sync::mpsc;

/// Engine Runner v2 (server-side computes)
///
/// 1. Start an MPSC listener.
/// 2. Listen for AddEngine command.
/// 3. Start engine in new thread.
/// 4. Periodically export engine states to file/database.
///
pub struct EngineRunnerV2 {
    rx: mpsc::Receiver<MpscMessage>,
}

impl EngineRunnerV2 {
    /// Create a new Engine-Runner-v2 instance.
    pub fn new(rx: mpsc::Receiver<MpscMessage>) -> Self {
        EngineRunnerV2 { rx }
    }

    /// Initialize MPSC Receive Loop.
    pub fn init(&mut self) {
        loop {
            self.receive();
        }
    }

    /// Receive MPSC Messages.
    pub fn receive(&mut self) {
        let msg = self.rx.recv().unwrap();
        println!("Server-side engine-runner received msg:\n\n{:?}", msg);
        match &msg {
            mpsc_msg::MpscMessage::AddEngine(engine_id, simulation_variant) => {
                self.add_engine(*engine_id, simulation_variant)
            }
        };
    }

    /// Add Engine.
    /// - Create new engine & run in separate thread.
    pub fn add_engine(&mut self, engine_id: usize, simulation_variant: &SimulationVariant) {
        let mut engine = SimulationEngineV2::new(engine_id);
        engine.init(Some(simulation_variant.clone()));
        std::thread::spawn(move || {
            engine.run();
        });
    }

    /// Stop & Remove Engine.
    pub fn remove_engine(&mut self, _engine_id: &usize) {
        // TODO stop compute-loop
        // - communicate with engines via MPSC?
        // TODO remove from database
        // println!("Engine-Runner removed engine {}", engine_id);
    }
}
