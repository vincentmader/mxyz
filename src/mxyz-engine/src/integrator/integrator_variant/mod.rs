pub mod field;
pub mod network;
pub mod object;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum IntegratorVariant {
    Field(field::FieldIntegratorVariant),
    Object(object::ObjectIntegratorVariant),
    Network(network::NetworkIntegratorVariant),
}
impl ToString for IntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::Field(integrator_variant) => integrator_variant.to_string(),
            Self::Object(integrator_variant) => integrator_variant.to_string(),
            Self::Network(integrator_variant) => integrator_variant.to_string(),
        }
    }
}
