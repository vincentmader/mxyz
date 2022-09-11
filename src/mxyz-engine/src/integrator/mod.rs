pub mod integrator_variant;
use crate::entity::Entity;
use crate::interaction::Interaction;
use crate::state::UnsizedState;
use integrator_variant::object::force::ForceIntegratorVariant;
use integrator_variant::object::ObjectIntegratorVariant;
use integrator_variant::IntegratorVariant;
use serde::{Deserialize, Serialize};

const DT: f64 = 0.01;

/// Integrator
///
/// Function:
/// - Match IntegratorVariant onto integrate function
/// - function takes in entity, state, interactions, & config
///
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
    pub fn forward_entity(
        &self,
        entity: ((usize, usize), &Box<dyn Entity>),
        state: &UnsizedState,
        config: &crate::config::EngineConfig,
    ) -> Box<dyn Entity> {
        let integrate = match &self.variant {
            IntegratorVariant::Object(integrator) => match integrator {
                ObjectIntegratorVariant::Force(integrator) => match integrator {
                    ForceIntegratorVariant::EulerExplicit => {
                        integrator_variant::object::force::euler::explicit::euler_explicit
                    }
                    _ => todo!("Match ObjectForceIntegratorVariant onto integrate function."),
                },
                _ => todo!("Match ForceIntegratorVariant onto integrate function."),
            },
            _ => todo!("Match IntegratorVariant onto integrate function."),
        };
        integrate(entity, state, &self.interactions, config)
    }
}
