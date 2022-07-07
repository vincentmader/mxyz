// #![allow(unreachable_patterns)]

// use mxyz_universe::entity::attribute::Position;
// use mxyz_universe::system::SystemVariant;

// pub fn _print_state(engine: &mxyz_engine::Engine) {
//     let state = &engine.states[engine.config.step_id.0];
//     println!("\n  Steps: {}", engine.config.step_id.0);
//     for system in state.systems.iter() {
//         match &system.variant {
//             SystemVariant::Planets(system) => {
//                 println!("objects");
//                 for e in system.entities.iter() {
//                     println!("    {:?}", e.get_position());
//                 }
//             }
//             // mxyz_universe::system::SystemVariant::DiscreteField => {
//             //     println!("field");
//             //     for e in system.entities.iter() {
//             //         println!("    {:?}", e.get_position()); // TODO
//             //     }
//             // }
//             _ => todo!(),
//         }
//     }
//     println!("");
// }

// pub fn _print_interaction_matrix(engine: &mxyz_engine::Engine) {
//     for (system_id, _system) in engine.states[engine.config.step_id.0]
//         .systems
//         .iter()
//         .enumerate()
//     {
//         let integrators = &engine.config.integrators[system_id];
//         for (integrator_id, integrator) in integrators.iter().enumerate() {
//             for (interaction_id, interaction) in integrator.interactions.iter().enumerate() {
//                 println!(
//                     "\n\nSystem {}, Integrator {}, Interaction {}: {:#?}",
//                     system_id, integrator_id, interaction_id, interaction.matrix
//                 );
//             }
//         }
//     }
// }
