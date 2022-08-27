use mxyz_engine_universe::entity::Entity;
use mxyz_engine_universe::entity::EntityV1;
use mxyz_engine_universe::interaction::force::ForceVariant;
use mxyz_engine_universe::interaction::Interaction;
use mxyz_engine_universe::interaction::InteractionVariant;
use mxyz_engine_universe::state::State;
use mxyz_engine_universe::system::{System, SystemVariant};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

const DT: f64 = 0.01;

pub fn apply(system: &System, states: &Vec<State>, interactions: &Vec<Interaction>) -> System {
    // Load current state.
    let current_state = &states[states.len() - 1]; // TODO: fix, this might fail

    // Copy over system-id, system-variant, & integrators.
    // - TODO find better way? (without clone)
    let system_id = system.system_id;
    let integrators = system.integrators.to_vec();
    let variant = system.variant.clone();

    let mut entities: Vec<Box<dyn Entity>> = vec![];
    let arc = Arc::new(Mutex::new(&mut entities));

    let _: Vec<()> = (0..system.entities.len())
        .into_par_iter()
        .map(|ent_id| {
            // Load entity from vector.
            let ent = &system.entities[ent_id];
            // Define next state's entity.
            let next_ent = ent.clone();
            //                   ^ TODO: Clone needed here?

            // Get entity's position & velocity.
            let position = next_ent.get_position();
            let velocity = next_ent.get_velocity();

            let mut acceleration = [0., 0., 0.];

            // TODO: choose from interaction matrix what to interact with

            // Loop over interacting systems.
            for (other_sys_id, other_sys) in current_state.systems.iter().enumerate() {
                // Loop over entities in interacting system.
                for (other_ent_id, other_ent) in other_sys.entities.iter().enumerate() {
                    // println!("{}-{} {}-{}", system_id, ent_id, other_sys_id, other_ent_id);

                    // Skip self-interaction.  (TODO: Handle through neighborhoods.)
                    if (system_id, ent_id) == (other_sys_id, other_ent_id) {
                        continue;
                    }
                    // println!("{}-{} {}-{}", system_id, ent_id, other_sys_id, other_ent_id);

                    // Loop over all interactions for this system pair & integrator
                    for interaction in interactions.iter() {
                        // TODO

                        apply_interaction(
                            interaction,
                            &next_ent,
                            ent,
                            other_ent,
                            &mut acceleration,
                        );
                    }
                }
            }

            // Forward velocity vector.
            let velocity = [
                velocity[0] + acceleration[0] * DT,
                velocity[1] + acceleration[1] * DT,
                velocity[2] + acceleration[2] * DT,
            ];
            // Forward position vector.
            let position = [
                position[0] + velocity[0] * DT,
                position[1] + velocity[1] * DT,
                position[2] + velocity[2] * DT,
            ];

            // Choose specific entity struct for this system's variant.
            let next_ent = match system.variant {
                SystemVariant::EntitiesV1 => EntityV1 {
                    mass: next_ent.get_mass(),
                    position,
                    velocity,
                },
                _ => todo!(),
            };
            // Push to next-state's entity vector.
            Arc::clone(&arc).lock().unwrap().push(Box::new(next_ent));
        })
        .collect();

    System {
        system_id,
        variant,
        integrators,
        entities,
    }
}

pub fn apply_interaction(
    interaction: &Interaction,
    next_ent: &Box<dyn Entity>,
    ent: &Box<dyn Entity>,
    other_ent: &Box<dyn Entity>,
    acceleration: &mut [f64; 3],
) {
    // println!("{:?} {:?}", ent.get_position(), other_ent.get_position());
    match &interaction.variant {
        InteractionVariant::Force(force) => {
            use crate::interaction::force::coulomb_interaction;
            use crate::interaction::force::newtonian_gravity;
            let force = match force.variant {
                ForceVariant::NewtonianGravity => newtonian_gravity::from(&ent, other_ent),
                ForceVariant::Coulomb => coulomb_interaction::from(&ent, other_ent),
                _ => todo!(),
            };
            // Update acceleration from force.
            acceleration[0] += force[0] / next_ent.get_mass();
            acceleration[1] += force[1] / next_ent.get_mass();
            acceleration[2] += force[2] / next_ent.get_mass();

            // println!("{:?}", force);
            // println!("{:?}", acceleration);
        }
        _ => todo!(),
    }
}
