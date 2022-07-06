use mxyz_config::SimulationVariant;
// use mxyz_network::package::Package;
// use std::sync::mpsc::Receiver;

pub struct ClientConfig {
    // pub engine_id: usize,
    pub simulation_variant: SimulationVariant,
    pub frame_id: (usize, usize),
    // pub rx: Receiver<Package>,
}
impl ClientConfig {
    pub fn new(
        // engine_id: usize,
        simulation_variant: SimulationVariant,
        // rx: Receiver<Package>
    ) -> Self {
        let frame_id = (0, usize::MAX);
        ClientConfig {
            // engine_id,
            simulation_variant,
            frame_id,
            // rx,
        }
    }
}
