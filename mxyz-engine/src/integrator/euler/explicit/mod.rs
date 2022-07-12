use mxyz_universe::entity::EntityV1;
use mxyz_universe::interaction::force::ForceVariant;
use mxyz_universe::interaction::Interaction;
use mxyz_universe::interaction::InteractionVariant;
use mxyz_universe::state::State;
use mxyz_universe::system::system::{System, SystemVariant};

const DT: f64 = 0.001;

pub fn apply(system: &System, states: &Vec<State>, interactions: &Vec<Interaction>) -> System {
    // Load current state.
    // - TODO might fail
    let current_state = &states[states.len() - 1];

    // Copy over system-id, system-variant, & integrators.
    // - TODO find better way? (without clone)
    let system_id = system.system_id;
    let integrators = system.integrators.to_vec();
    let variant = system.variant.clone();

    // Define system for next time-step. (this will be returned)
    let entities = vec![];
    let mut next_system = System {
        system_id,
        variant,
        integrators,
        entities,
    };

    // Loop over entities in system.
    for (ent_id, ent) in system.entities.iter().enumerate() {
        // TODO think re: clone needed here?
        let next_ent = ent.clone();

        // Get entity's position & velocity.
        let mass = next_ent.get_mass();
        let position = next_ent.get_position();
        let velocity = next_ent.get_velocity();

        // Loop over interacting systems.
        // - TODO choose from interaction matrix what to interact with
        let mut acceleration = [0., 0., 0.];
        for (other_sys_id, other_sys) in current_state.systems.iter().enumerate() {
            // Loop over entities in interacting system.
            for (other_ent_id, other_ent) in other_sys.entities.iter().enumerate() {
                // Skip self-interaction.
                // - TODO handle through neighborhoods.
                if (system_id, ent_id) == (other_sys_id, other_ent_id) {
                    continue;
                }
                // Loop over all interactions for this system pair & integrator
                for interaction in interactions {
                    // TODO
                    match &interaction.variant {
                        InteractionVariant::Force(force) => {
                            let force = match force.variant {
                                ForceVariant::NewtonianGravity => {
                                    crate::interaction::force::newtonian_gravity::calculate_from(
                                        ent, other_ent,
                                    )
                                }
                                _ => todo!(),
                            };

                            acceleration[0] += force[0] / mass;
                            acceleration[1] += force[1] / mass;
                            acceleration[2] += force[2] / mass;
                            // println!("{}", position[0] / acceleration[0]);
                        }
                        _ => todo!(),
                    }
                }
            }
        }
        let velocity = [
            velocity[0] + acceleration[0] * DT,
            velocity[1] + acceleration[1] * DT,
            velocity[2] + acceleration[2] * DT,
        ];
        let position = [
            position[0] + velocity[0] * DT,
            position[1] + velocity[1] * DT,
            position[2] + velocity[2] * DT,
        ];

        // TODO think re: how to return?
        // - has to be specific Type
        let res = match system.variant {
            SystemVariant::EntitiesV1 => {
                let mass = ent.get_mass();
                let position = position;
                let velocity = velocity;
                //
                EntityV1 {
                    mass,
                    position,
                    velocity,
                }
            }
            _ => todo!(),
        };
        let res = Box::new(res);
        next_system.entities.push(res)
    }

    next_system
}
