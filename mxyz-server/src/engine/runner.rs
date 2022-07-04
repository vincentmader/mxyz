use mxyz_engine::state::preset::SimulationVariant;
use mxyz_engine::Engine;
use mxyz_network::message::Message;
use mxyz_network::mpsc::MpscReceiver;
use std::sync::mpsc;

pub struct EngineRunner {
    mpsc_receiver: MpscReceiver,
    engines: Vec<Engine>,
}
impl EngineRunner {
    pub fn new(mpsc_receiver: MpscReceiver) -> Self {
        let engines = vec![];
        EngineRunner {
            engines,
            mpsc_receiver,
        }
    }
    pub fn init(&mut self) {
        println!("Initializing Engine Runner...");
        loop {
            self.receive();
        }
    }
    pub fn receive(&mut self) {
        println!("Engine Runner listening...");
        let msg = self.mpsc_receiver.rx.recv().unwrap();
        println!("Engine Runner received msg: {:?}", msg);
        match &msg {
            Message::AddEngine(simulation_variant) => self.add_engine(simulation_variant),
            Message::RemoveEngine(engine_id) => self.remove_engine(engine_id),
        };
        println!("Server received MPSC msg: {:#?}", msg);
    }
    pub fn add_engine(&mut self, simulation_variant: &SimulationVariant) {
        let engine_id = 0;
        //...
        println!("Engine-Runner added engine {}", engine_id);
    }
    pub fn remove_engine(&mut self, engine_id: &usize) {
        // ...
        println!("Engine-Runner removed engine {}", engine_id);
    }
}
