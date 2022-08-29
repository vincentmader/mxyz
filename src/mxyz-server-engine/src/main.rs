use mxyz_engine::engine::Engine;
use mxyz_engine_universe::preset::SimulationVariant;
use mxyz_server_engine::SimulationEngineV2Server;

fn main() {
    let engine_id = 0;
    let mut engine = SimulationEngineV2Server::new(engine_id);
    engine.init(Some(SimulationVariant::ThreeBodyMoon));
    engine.run();
}
