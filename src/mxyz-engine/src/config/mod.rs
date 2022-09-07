pub mod constants;
pub mod engine_runner_variant;
pub mod export_variant;
pub mod preset;
pub mod simulation_variant;
use crate::integrator::Integrator;
use crate::interaction::Interaction;
use export_variant::ExportVariant;
use simulation_variant::SimulationVariant;
use std::collections::HashMap;

pub struct EngineConfig {
    pub step_id: (usize, usize),
    // Database
    pub export_variant: ExportVariant,
    pub nr_of_steps_between_exports: usize,
    pub last_export_step_id: Option<usize>,
    // Preset
    pub simulation_variant: Option<SimulationVariant>,
    //
    pub interactions: Vec<Interaction>,
    pub systems: HashMap<usize, SystemConfig>,
    //
    // pub constants: Constants,
}
impl EngineConfig {
    pub fn new() -> Self {
        let step_id = (0, usize::MAX);
        let export_variant = ExportVariant::ToDatabase;
        let last_export_step_id = None;
        let nr_of_steps_between_exports = 100;
        let simulation_variant = None;
        // let constants = Constants::new();
        let interactions = vec![];
        let systems = HashMap::new();

        EngineConfig {
            step_id,
            export_variant,
            last_export_step_id,
            nr_of_steps_between_exports,
            simulation_variant,
            // constants,
            interactions,
            systems,
        }
    }
}

pub struct SystemConfig {
    pub integrators: Vec<Integrator>,
}
impl SystemConfig {
    pub fn new() -> Self {
        let integrators = vec![];
        SystemConfig { integrators }
    }
}
