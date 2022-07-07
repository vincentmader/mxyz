use crate::interaction::Interaction;
use serde::{Deserialize, Serialize};

// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Integrator {
    pub variant: IntegratorVariant,
    pub interactions: Vec<Interaction>,
}

// ============================================================================

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
