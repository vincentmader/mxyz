pub mod integrator_variant;
use crate::entity::Entity;
use crate::interaction::Interaction;
use crate::neighborhoods::NeighborhoodVariant;
use crate::neighborhoods::Neighborhoods;
use crate::state::UnsizedState;
use integrator_variant::object::force::ForceIntegratorVariant;
use integrator_variant::object::ObjectIntegratorVariant;
use integrator_variant::IntegratorVariant;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

<<<<<<< HEAD
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InteractionMatrix(HashMap<usize, HashMap<usize, NeighborhoodVariant>>); // todo: move to vec-vec?;

impl InteractionMatrix {
    pub fn new() -> Self {
        InteractionMatrix(HashMap::new())
    }

    pub fn set_neighborhood_variant(
        &mut self,
        system_id: usize,
        other_id: usize,
        neighborhood: NeighborhoodVariant,
    ) {
        let interaction_matrix = &mut self.0;
        let _ = interaction_matrix
            .entry(system_id)
            .or_insert(HashMap::new());
        interaction_matrix
            .get_mut(&system_id)
            .unwrap()
            .insert(other_id, neighborhood);
    }

    pub fn get_neighborhood_variant(
        &self,
        system_id: usize,
        other_id: usize,
    ) -> NeighborhoodVariant {
        match self.0.get(&system_id) {
            Some(x) => match x.get(&other_id) {
                Some(x) => x.clone(), // TODO remove clone
                None => NeighborhoodVariant::None,
            },
            None => NeighborhoodVariant::None,
        }
    }
}

=======
>>>>>>> parent of f327976 (continued on integrators)
/// Integrator
/// - Integrator Variant
/// - Interaction Matrix
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
    pub matrix: HashMap<usize, HashMap<usize, bool>>, // todo: move to vec-vec?

                                                      // pub neighborhood: NeighborhoodVariant,
}
impl Integrator {
    /// Create new Integrator.
    /// - Default neighborhood-variant to All, i.e. O(N^2) nested loop over all.
    pub fn new(variant: IntegratorVariant) -> Self {
        let neighborhood = NeighborhoodVariant::All; // TODO move to new() args?
        Integrator {
            variant,
            // neighborhood,
            interactions: vec![],
            matrix: HashMap::new(),
        }
    }
    pub fn forward_entity(
        &self,
        entity: ((usize, usize), &Box<dyn Entity>),
        state: &UnsizedState,
        config: &crate::config::EngineConfig,
        neighborhoods: &Neighborhoods,
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
        let (interactions, matrix) = (&self.interactions, &self.matrix);

        // TODO neighborhood -> neighborhoods
        integrate(entity, state, interactions, neighborhoods, matrix, config)
    }
}
