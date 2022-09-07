pub mod collision;
pub mod force;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Entity Integrator
pub enum ObjectIntegratorVariant {
    ForceIntegrator(force::ForceIntegratorVariant),
    Collision(collision::CollisionIntegratorVariant),
}

impl ToString for ObjectIntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::ForceIntegrator(x) => x.to_string(),
            Self::Collision(x) => x.to_string(),
        }
    }
}
