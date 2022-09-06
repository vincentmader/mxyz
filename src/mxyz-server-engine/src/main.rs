use mxyz_engine::config::simulation_variant::SimulationVariant;
use mxyz_engine::Engine;
use mxyz_server_engine::engine::SimulationEngineV2;

fn main() {
    let engine_id = 0;
    let mut engine = SimulationEngineV2::new(engine_id);
    engine.init(Some(SimulationVariant::ThreeBodyMoon));
    // engine.run();
}
