pub mod bulirsch_stoer;
pub mod euler;
pub mod lax_friedrichs;
pub mod lax_wendroff;
pub mod leapfrog;
pub mod runge_kutta;
pub mod velocity_verlet;
pub mod verlet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ForceIntegratorVariant {
    // EulerExplicit(euler::explicit::EulerExplicitObjectForceIntegrator),
    EulerExplicit,
    // EulerImplicit,
    // RungeKutta2,
    // RungeKutta4,
    // RungeKuttaN,
    // VelocityVerlet,
    // Verlet,
    // LeapFrog,
    // BulirschStoer,
}
impl ToString for ForceIntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::EulerExplicit => "Explicit Euler".into(),
            // Self::EulerImplicit => "Implicit Euler",
            // Self::RungeKutta2 => "2nd Order Runge-Kutta",
            // Self::RungeKutta4 => "4th Order Runge-Kutta",
        }
    }
}
