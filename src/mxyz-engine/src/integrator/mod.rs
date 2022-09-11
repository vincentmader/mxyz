pub mod integrator_variant;
use crate::entity::Entity;
use crate::interaction::Interaction;
use crate::neighborhoods::NeighborhoodVariant;
use crate::state::UnsizedState;
use integrator_variant::object::force::ForceIntegratorVariant;
use integrator_variant::object::ObjectIntegratorVariant;
use integrator_variant::IntegratorVariant;
use serde::{Deserialize, Serialize};

/// Integrator
///
/// Function:
/// - Match IntegratorVariant onto integrate function
/// - function takes in
///   - entity,
///   - state,
///   - interactions,
///   - neighborhood-variant, &
///   - config.
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Integrator {
    pub variant: IntegratorVariant,
    pub interactions: Vec<Interaction>,
    pub neighborhood: NeighborhoodVariant,
}
impl Integrator {
    /// Create new Integrator.
    /// - Default neighborhood-variant to All, i.e. O(N^2) nested loop over all.
    pub fn new(variant: IntegratorVariant) -> Self {
        let interactions = vec![];
        let neighborhood = NeighborhoodVariant::All; // TODO move to new() args?
        Integrator {
            variant,
            interactions,
            neighborhood,
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
        let (interactions, neighborhood) = (&self.interactions, &self.neighborhood);
        integrate(entity, state, interactions, neighborhood, config)
    }
}
