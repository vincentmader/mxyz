use mxyz_engine::config::simulation_variant::SimulationVariant;
use mxyz_engine::state::StateQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Request: Add new Client.
    AddClient,

    /// Request: Add new Engine.
    /// - client-id
    /// - simulation-variant
    AddEngine(usize, SimulationVariant),

    /// Request: Remove Engine.
    /// - engine-id
    RemoveEngine(usize),

    /// Request: Get States.
    /// - engine-id
    /// - state-query
    GetUpdatedStates(usize, StateQuery),
}
