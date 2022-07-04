use mxyz_engine::Engine;
use mxyz_network::package::request::Request;
use mxyz_network::package::Package;
use mxyz_universe::preset::SimulationVariant;
use std::sync::mpsc;

pub struct EngineRunner {
    rx: mpsc::Receiver<Package>,
    engines: Vec<Engine>,
}
impl EngineRunner {
    pub fn new(rx: mpsc::Receiver<Package>) -> Self {
        let engines = vec![];
        EngineRunner { rx, engines }
    }
    pub fn init(&mut self) {
        println!("Initializing Engine Runner...");
        loop {
            self.receive();
        }
    }
    pub fn receive(&mut self) {
        println!("Engine Runner listening...");
        let msg = self.rx.recv().unwrap();
        println!("Engine Runner received msg: {:?}", msg);
        match &msg {
            Package::Request(req) => match req {
                Request::AddEngine(simulation_variant) => self.add_engine(simulation_variant),
                Request::RemoveEngine(engine_id) => self.remove_engine(engine_id),
                _ => todo!(),
            },
            Package::Response(res) => match res {
                _ => todo!(),
            },
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
