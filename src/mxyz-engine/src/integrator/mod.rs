pub mod integrator_variant;
use crate::entity::Entity;
use crate::interaction::force::ForceVariant;
use crate::interaction::Interaction;
use crate::interaction::InteractionVariant;
use crate::state::UnsizedState;
use integrator_variant::field::FieldIntegratorVariant;
use integrator_variant::object::force::ForceIntegratorVariant;
use integrator_variant::object::ObjectIntegratorVariant;
use integrator_variant::IntegratorVariant;
use serde::{Deserialize, Serialize};

const DT: f64 = 0.01;

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
    ) -> Box<dyn Entity> {
        // println!("\n\n{:?}", entity);
        let (entity_id, entity) = entity;
        let interactions = &self.interactions;
        // ...
        let (m, pos, vel, q) = (
            entity.get_mass(),
            entity.get_position(),
            entity.get_velocity(),
            entity.get_charge(),
        );

        match &self.variant {
            IntegratorVariant::Object(object_integrator_variant) => match object_integrator_variant
            {
                ObjectIntegratorVariant::ForceIntegrator(force_integrator_variant) => {
                    match force_integrator_variant {
                        ForceIntegratorVariant::EulerExplicit => {
                            let mut total_force = [0., 0., 0.];
                            // Loop over systems.
                            for (system_id, system) in state.systems.iter().enumerate() {
                                // Define neighborhood.
                                // - TODO
                                // Loop over entities in other system.
                                for (other_id, other) in system.entities.iter().enumerate() {
                                    let other_id = (system_id, other_id);
                                    if entity_id == (other_id) {
                                        continue;
                                    }
                                    for interaction in interactions.iter() {
                                        let get_force = match &interaction.variant {
                                    InteractionVariant::Force(force) => match force.variant {
                                        ForceVariant::NewtonianGravity => {
                                            crate::interaction::force::newtonian_gravity::from
                                        }
                                        ForceVariant::Coulomb => {
                                            crate::interaction::force::coulomb_interaction::from
                                        }
                                        _ => todo!("Force Interaction Variant"),
                                    },
                                    _ => todo!("Interaction Variant"),
                                };
                                        let force = get_force(entity, other);
                                        total_force = [
                                            total_force[0] + force[0],
                                            total_force[1] + force[1],
                                            total_force[2] + force[2],
                                        ];
                                    }
                                }
                            }
                            let acc: Vec<f64> = (0..3).map(|i| total_force[i] / m).collect();
                            let vel = entity.get_velocity();
                            let vel: Vec<f64> = (0..3).map(|i| vel[i] + acc[i] * DT).collect();
                            let pos: Vec<f64> = (0..3).map(|i| pos[i] + vel[i] * DT).collect();
                            let pos = [pos[0], pos[1], pos[2]];
                            let vel = [vel[0], vel[1], vel[2]];
                            let entity = crate::entity::entity_v1::EntityV1::new(m, pos, vel, q);
                            Box::new(entity)
                        }
                        _ => todo!("Object Force Integrator Variant"),
                    }
                }
                _ => todo!("Object Integrator Variant"),
            },
            _ => todo!("Integrator Variant"),
        }
    }
}
