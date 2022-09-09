use mxyz_engine::config::simulation_variant::SimulationVariant;

#[derive(Debug)]
pub enum MpscMessage {
    /// Message: Add Engine.
    /// - engine-id
    /// - simulation-variant
    AddEngine(usize, SimulationVariant),
}
