use mxyz_engine::config::simulation_variant::SimulationVariant;
use mxyz_engine::Engine;
use mxyz_engine_v2::SimulationEngineV2;

const ENGINE_ID: usize = 0;
const SIMULATION_VARIANT: SimulationVariant = SimulationVariant::ThreeBodyMoon;

fn main() {
    let mut engine = SimulationEngineV2::new(ENGINE_ID);
    engine.init(Some(SIMULATION_VARIANT));
}
