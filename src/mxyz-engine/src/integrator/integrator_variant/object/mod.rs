pub mod collision;
pub mod force;
use force::ForceIntegratorVariant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Entity Integrator
pub enum ObjectIntegratorVariant {
    ForceIntegrator(force::ForceIntegratorVariant),
    Collision(collision::CollisionIntegratorVariant),
}
