use serde::{Deserialize, Serialize};
pub mod cellular_automata;
pub mod monte_carlo;

// TODO
// - think re: what integrator variants are there for fields?
// - might also use object-force integration schemes,
//   only implemented slightly differently
// -> might have to rebuild this module & super

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Entity Integrator
pub enum FieldIntegratorVariant {
    // CellularAutomaton,
    // MonteCarlo,
}
