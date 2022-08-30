#![allow(unreachable_patterns)]
pub mod three_body_figure_eight;
pub mod three_body_moon;
use crate::config::simulation_variant::SimulationVariant;
use crate::config::EngineConfig;
use crate::state::State;

/// Initialize State & Config
pub fn initialize(simulation_variant: Option<SimulationVariant>, cfg: &mut EngineConfig) -> State {
    let mut state = State::new(0);
    let mut systems = vec![];
    match simulation_variant {
        Some(simulation_variant) => match simulation_variant {
            SimulationVariant::ThreeBodyMoon => {
                three_body_moon::preset(&mut systems, cfg);
                state.systems = systems;
                state
            }
            _ => todo!("{:?}", simulation_variant),
        },
        None => todo!("handle this earlier? (in str->enum sim-var conversion)"),
    }
}
