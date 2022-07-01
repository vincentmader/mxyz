use crate::entity::attribute::*;
use crate::entity::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellularAutomaton {}
impl CellularAutomaton {
    pub fn new() -> Self {
        CellularAutomaton {}
    }
}
impl Entity for CellularAutomaton {}
impl Force for CellularAutomaton {}
impl Position for CellularAutomaton {}
impl Charge for CellularAutomaton {}
impl Mass for CellularAutomaton {}
impl Velocity for CellularAutomaton {}
impl Density for CellularAutomaton {}
