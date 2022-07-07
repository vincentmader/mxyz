pub struct EngineConfig {
    pub step_id: (usize, usize),
}
impl EngineConfig {
    pub fn new() -> Self {
        let step_id = (0, usize::MAX);
        // let interactions = vec![];
        // let integrators = vec![];
        // let export_variant = ExportVariant::ToDatabase;
        // let last_export_step_id = None;
        // let nr_of_steps_between_exports = 100;
        // let simulation_variant = None;
        EngineConfig {
            step_id,
            // systems,
            // interactions,
            // integrators,
            // export_variant,
            // last_export_step_id,
            // nr_of_steps_between_exports,
            // simulation_variant,
        }
    }
}

pub struct ClientConfig {}
pub struct RendererConfig {}
