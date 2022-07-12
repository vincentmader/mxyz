use mxyz_universe::entity::EntityV1;
use mxyz_universe::interaction::Interaction;
use mxyz_universe::state::State;
use mxyz_universe::system::system::{System, SystemVariant};
use mxyz_universe::system::EntitiesV1;

pub fn apply(
    sys_1: &System,
    current_state: &State,
    states: &Vec<State>,
    interactions: &Vec<Interaction>,
) -> System {
    let system_id = sys_1.system_id;
    let integrators = sys_1.integrators.to_vec();
    let variant = sys_1.variant.clone(); // TODO find better way?
    let entities = vec![];
    let mut system = System {
        system_id,
        variant,
        integrators,
        entities,
    };

    // Loop over entities in system.
    for e_1 in sys_1.entities.iter() {
        // TODO think re: clone here?
        // TODO
        // TODO
        // TODO

        // Get entity's position & velocity.
        let position = e_1.get_position();
        let velocity = e_1.get_position();

        // Loop over interacting systems.
        // - TODO choose from interaction matrix what to interact with
        for sys_2 in current_state.systems.iter() {
            // Loop over entities in interacting system.
            for e_2 in sys_2.entities.iter() {
                // Loop over all interactions for this system pair & integrator
                for interaction in interactions {
                    // TODO
                    match interaction.variant {
                        _ => todo!(),
                    }
                }
            }
        }
        // TODO think re: how to return?
        // - has to be specific Type
        let res = match sys_1.variant {
            SystemVariant::EntitiesV1 => {
                let mass = e_1.get_mass();
                let position = *e_1.get_position();
                let velocity = *e_1.get_velocity();
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
        system.entities.push(res)
    }

    //let entities = sys_1
    //    .entities
    //    .iter()
    //    .map(|e| {
    //        let res = match sys_1.variant {
    //            SystemVariant::EntitiesV1 => {
    //                let mass = e.get_mass();
    //                let position = *e.get_position();
    //                let velocity = *e.get_velocity();
    //                //
    //                EntityV1 {
    //                    mass,
    //                    position,
    //                    velocity,
    //                }
    //            }
    //        };
    //        // for sys_2 in current_state.systems.iter() {
    //        //     for e_2 in sys_2.entities.iter() {}
    //        // }
    //        // e.clone()
    //        Box::new(res)
    //    })
    //    .collect();

    // System::new(sys_1.system_id, sys_1.variant.clone())
    // System {
    //     system_id,
    //     integrators: integrators.to_vec(),
    //     entities,
    //     variant,
    // }
    system
}
