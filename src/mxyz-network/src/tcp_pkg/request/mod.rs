use mxyz_engine_universe::preset::SimulationVariant;
use mxyz_engine_universe::state::StateQuery;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    /// Adds Client
    AddClient,
    /// Adds Engine with given Client-ID & SimulationVariant
    AddEngine(usize, SimulationVariant),
    /// Removes Engine
    RemoveEngine(usize),
    /// Gets States since last Update
    GetUpdatedStates(usize, StateQuery), // engine-id, state-id
}
