pub mod constants;
pub mod engine_runner_variant;
pub mod export_variant;
pub mod preset;
pub mod simulation_variant;
use export_variant::ExportVariant;
use serde::{Deserialize, Serialize};
use simulation_variant::SimulationVariant;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EngineConfig {
    pub step_id: (usize, u128), // todo: is step_id.1 even used?
    pub simulation_variant: Option<SimulationVariant>,
    pub last_export: Option<usize>,
    pub export_variant: ExportVariant,
    pub nr_of_steps_between_exports: usize,
}

impl EngineConfig {
    pub fn new() -> Self {
        EngineConfig {
            step_id: (0, u128::MAX),
            simulation_variant: None,
            last_export: None,
            export_variant: ExportVariant::ToDatabase,
            nr_of_steps_between_exports: 100,
        }
    }
}
