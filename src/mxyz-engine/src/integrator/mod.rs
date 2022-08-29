pub mod bulirsch_stoer;
pub mod cellular_automata;
pub mod collision;
pub mod euler;
pub mod lax_friedrichs;
pub mod lax_wendroff;
pub mod leapfrog;
pub mod monte_carlo;
pub mod runge_kutta;
pub mod velocity_verlet;
pub mod verlet;
// -----------------------------------------------------------------------------
use crate::interaction::Interaction;
use serde::{Deserialize, Serialize};
// -----------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
/// Entity Integrator
pub enum IntegratorVariant {
    EulerExplicit,
    EulerImplicit,
    RungeKutta2,
    RungeKutta4,
    RungeKuttaN,
    VelocityVerlet,
    Verlet,
    LeapFrog,
    BulirschStoer,
    Collision,
    CellularAutomaton,
    MonteCarlo,
}
// -----------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Integrator {
    pub variant: IntegratorVariant,
    pub interactions: Vec<Interaction>,
}
impl Integrator {
    pub fn new(variant: IntegratorVariant) -> Self {
        let interactions = vec![];
        Integrator {
            variant,
            interactions,
        }
    }
}
