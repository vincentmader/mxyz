use mxyz_engine_universe::preset::SimulationVariant;

pub struct EngineConfig {
    pub step_id: (usize, usize),
    pub simulation_variant: Option<SimulationVariant>,
    // pub systems: Vec<SystemConfig>,
    // pub interactions: Vec<Interaction>,
    // pub integrators: Vec<Vec<Integrator>>,
    // pub constants: Constants,
    pub export_variant: ExportVariant,
    pub last_export_step_id: Option<usize>,
    pub nr_of_steps_between_exports: usize,
}
impl EngineConfig {
    pub fn new() -> Self {
        // let systems = vec![];
        // let interactions = vec![];
        // let integrators = vec![];
        let step_id = (0, usize::MAX);
        // let constants = Constants::new();
        let export_variant = ExportVariant::ToDatabase;
        let last_export_step_id = None;
        let nr_of_steps_between_exports = 100;
        let simulation_variant = None;
        EngineConfig {
            // systems,
            // interactions,
            // integrators,
            step_id,
            // constants,
            export_variant,
            last_export_step_id,
            nr_of_steps_between_exports,
            simulation_variant,
        }
    }
}

pub enum ExportVariant {
    ToDatabase,
    ToFile,
}
