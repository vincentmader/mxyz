use crate::config::EngineConfig;
use crate::entity::Entity;
use crate::integrator::InteractionMatrix;
use crate::interaction::interaction_variant::force::ForceVariant;
use crate::interaction::interaction_variant::InteractionVariant;
use crate::interaction::Interaction;
use crate::neighborhoods::NeighborhoodVariant;
use crate::neighborhoods::Neighborhoods;
use crate::state::UnsizedState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EulerExplicitObjectForceIntegrator {}

const DT: f64 = 0.01;

pub fn euler_explicit(
    entity: ((usize, usize), &Box<dyn Entity>),
    state: &UnsizedState,
    interactions: &Vec<Interaction>,
    neighborhoods: &Neighborhoods,
    matrix: &InteractionMatrix,
    _config: &EngineConfig,
) -> Box<dyn Entity> {
    use crate::interaction::interaction_variant::force;
    let (entity_id, entity) = entity;

    // Loop over systems, calculate over-all force acting on entity.
    let mut total_force = [0., 0., 0.];
    for (system_id, system) in state.systems.iter().enumerate() {
        // Skip system if interaction-matrix entry for integrator is equal to NeighboorhoodVariant::None (TODO)
        let neighborhood = matrix.get_neighborhood_variant(entity_id.0, system_id);
        let neighborhood = match neighborhood {
            NeighborhoodVariant::All => (0..system.entities.len()).collect::<Vec<usize>>(),
            NeighborhoodVariant::Sectors(_) => vec![1, 2, 3], // TODO
            NeighborhoodVariant::None => continue,
        };

        // Loop over entities in other system.
        for other_id in neighborhood.iter() {
            let other = system.entities.get(*other_id).unwrap();
            let other_id = (system_id, *other_id);
            if entity_id == (other_id) {
                continue;
            }
            for interaction in interactions.iter() {
                // if !interaction.matrix.entries[system_id].unwrap() {
                //     continue;
                // }
                let get_force = match &interaction.variant {
                    InteractionVariant::Force(force) => match force.variant {
                        ForceVariant::NewtonianGravity => force::newtonian_gravity::from,
                        ForceVariant::Coulomb => force::coulomb_interaction::from,
                        ForceVariant::LennardJones => force::lennard_jones_interaction::from,
                        _ => todo!("Force Interaction Variant"),
                    },
                    _ => todo!("Interaction Variant"),
                };
                let force = get_force(entity, other, &system.entities);
                total_force = [
                    total_force[0] + force[0],
                    total_force[1] + force[1],
                    total_force[2] + force[2],
                ];
            }
        }
    }
    let (m, pos, vel, q) = (
        entity.get_mass(),
        entity.get_position(),
        entity.get_velocity(),
        entity.get_charge(),
    );
    let acc: Vec<f64> = (0..3).map(|i| total_force[i] / m).collect();
    let vel: Vec<f64> = (0..3).map(|i| vel[i] + acc[i] * DT).collect();
    let pos: Vec<f64> = (0..3).map(|i| pos[i] + vel[i] * DT).collect();
    let pos = [pos[0], pos[1], pos[2]];
    let vel = [vel[0], vel[1], vel[2]];
    let entity = crate::entity::entity_v1::EntityV1::new(m, pos, vel, q);
    Box::new(entity)
}

// pub fn apply(
//     system: &UnsizedSystem,
//     states: &Vec<UnsizedState>,
//     interactions: &Vec<Interaction>,
// ) -> UnsizedSystem {
//     // Load current state.
//     let current_state = &states[states.len() - 1]; // TODO: fix, this might fail

//     // Copy over system-id, system-variant, & integrators.
//     // - TODO find better way? (without clone)
//     let system_id = system.system_id;
//     let integrators = system.integrators.to_vec();
//     let variant = system.variant.clone();

//     let mut entities: Vec<Box<dyn Entity>> = vec![];
//     let arc = Arc::new(Mutex::new(&mut entities));

//     let _: Vec<()> = (0..system.entities.len())
//         .into_par_iter()
//         .map(|ent_id| {
//             // Load entity from vector.
//             let ent = &system.entities[ent_id];
//             // Define next state's entity.
//             let next_ent = ent.clone();
//             //                   ^ TODO: Clone needed here?

//             // Get entity's position & velocity.
//             let position = next_ent.get_position();
//             let velocity = next_ent.get_velocity();

//             let mut acceleration = [0., 0., 0.];

//             // TODO: choose from interaction matrix what to interact with

//             // Loop over interacting systems.
//             for (other_sys_id, other_sys) in current_state.systems.iter().enumerate() {
//                 // if !
//                 // Loop over entities in interacting system.
//                 for (other_ent_id, other_ent) in other_sys.entities.iter().enumerate() {
//                     // println!("{}-{} {}-{}", system_id, ent_id, other_sys_id, other_ent_id);

//                     // Skip self-interaction.  (TODO: Handle through neighborhoods.)
//                     if (system_id, ent_id) == (other_sys_id, other_ent_id) {
//                         continue;
//                     }
//                     // println!("{}-{} {}-{}", system_id, ent_id, other_sys_id, other_ent_id);

//                     // Loop over all interactions for this system pair & integrator
//                     for interaction in interactions.iter() {
//                         // TODO

//                         apply_interaction(
//                             interaction,
//                             &next_ent,
//                             ent,
//                             other_ent,
//                             &mut acceleration,
//                         );
//                     }
//                 }
//             }

//             // Forward velocity vector.
//             let velocity = [
//                 velocity[0] + acceleration[0] * DT,
//                 velocity[1] + acceleration[1] * DT,
//                 velocity[2] + acceleration[2] * DT,
//             ];
//             // Forward position vector.
//             let position = [
//                 position[0] + velocity[0] * DT,
//                 position[1] + velocity[1] * DT,
//                 position[2] + velocity[2] * DT,
//             ];

//             // Choose specific entity struct for this system's variant.
//             let next_ent = match system.variant {
//                 UnsizedSystemVariant::EntitiesV1 => EntityV1 {
//                     mass: next_ent.get_mass(),
//                     position,
//                     velocity,
//                     charge: next_ent.get_charge(),
//                 },
//                 _ => todo!(),
//             };
//             // Push to next-state's entity vector.
//             Arc::clone(&arc).lock().unwrap().push(Box::new(next_ent));
//         })
//         .collect();

//     UnsizedSystem {
//         system_id,
//         variant,
//         integrators,
//         entities,
//     }
// }

// pub fn apply_interaction(
//     interaction: &Interaction,
//     next_ent: &Box<dyn Entity>,
//     ent: &Box<dyn Entity>,
//     other_ent: &Box<dyn Entity>,
//     acceleration: &mut [f64; 3],
// ) {
//     // println!("{:?} {:?}", ent.get_position(), other_ent.get_position());
//     match &interaction.variant {
//         InteractionVariant::Force(force) => {
//             use crate::interaction::force::coulomb_interaction;
//             use crate::interaction::force::newtonian_gravity;
//             let force = match force.variant {
//                 ForceVariant::NewtonianGravity => newtonian_gravity::from(&ent, other_ent),
//                 ForceVariant::Coulomb => coulomb_interaction::from(&ent, other_ent),
//                 _ => todo!(),
//             };
//             // Update acceleration from force.
//             acceleration[0] += force[0] / next_ent.get_mass();
//             acceleration[1] += force[1] / next_ent.get_mass();
//             acceleration[2] += force[2] / next_ent.get_mass();

//             // println!("{:?}", force);
//             // println!("{:?}", acceleration);
//         }
//         _ => todo!(),
//     }
// }
