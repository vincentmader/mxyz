use crate::entity::field::cellular_automaton::CellularAutomaton;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularAutomata {
    entities: Vec<CellularAutomaton>,
}
impl CellularAutomata {
    pub fn new() -> Self {
        let entities = vec![];
        CellularAutomata { entities }
    }
}
