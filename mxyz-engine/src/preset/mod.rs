#![allow(unreachable_patterns)]
pub mod three_body_figure_eight;
pub mod three_body_moon;
use mxyz_config::EngineConfig;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::state::State;

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
            _ => todo!(),
        },
        None => todo!("handle this earlier? (in str->enum sim-var conversion)"),
    }

    // let mut systems = vec![];
    // match &simulation_variant {
    //     None => {}
    //     Some(id) => match id {
    //         SimulationVariant::ThreeBodyFigureEight => {
    //             three_body_figure_eight::preset(&mut systems, config)
    //         }
    //         SimulationVariant::ThreeBodyMoon => three_body_moon::preset(&mut systems, config),
    //         _ => todo!(),
    //     },
    // }
    // config.simulation_variant = simulation_variant;
    // systems
}
