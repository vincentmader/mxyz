pub mod field;
pub mod network;
pub mod object;
use serde::{Deserialize, Serialize};

/// Integrator Variant
///
/// Types:
/// - Discrete Field
/// - Family of Particles
/// - Neural Network (later)
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IntegratorVariant {
    Field(field::FieldIntegratorVariant),
    Object(object::ObjectIntegratorVariant),
    Network(network::NetworkIntegratorVariant),
}

impl ToString for IntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Field(x) => x.to_string(),
            Self::Object(x) => x.to_string(),
            Self::Network(x) => x.to_string(),
        }
    }
}
