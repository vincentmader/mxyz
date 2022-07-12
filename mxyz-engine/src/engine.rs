#![allow(unreachable_code)]
use crate::integrator::euler;
use crate::preset;
use mxyz_config::EngineConfig;
use mxyz_universe::integrator::Integrator;
use mxyz_universe::integrator::IntegratorVariant;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::state::State;
use mxyz_universe::system::system::System;

// ============================================================================

/// MXYZ Simulation Engine
pub struct Engine {
    pub engine_id: usize,
    pub config: EngineConfig,
    pub states: Vec<State>,
}
impl Engine {
    /// Creates a new Engine instance
    pub fn new(engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];
        Engine {
            engine_id,
            config,
            states,
        }
    }

    /// Initializes State & Config
    pub fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        let initial_state = preset::initialize(sim_variant, &mut self.config);
        self.states.push(initial_state);
    }

    /// Runs Engine
    pub fn run(&mut self) {
        println!("MXYZ-Engine: Running...");
        // Run compute-loop for nr. of steps specified in engine configuration.
        for _ in 0..self.config.step_id.1 {
            // Forward engine to next time-step.
            self.forward()
        }
    }

    /// Forwards Engine
    pub fn forward(&mut self) {
        // Forward state to next time-step.
        let next_state = self.forward_state();
        self.states.push(next_state);
        // Update step-id.
        self.config.step_id.0 += 1;
    }

    /// Forwards State
    pub fn forward_state(&self) -> State {
        // Load current state.
        let current_state = &self.states[self.config.step_id.0];

        // TODO Build trees/neighborhoods for all systems.

        // Forward systems.
        let systems = current_state
            .systems
            .iter()
            .map(|sys| self.forward_system(sys))
            .collect();
        // Update state-id.
        let state_id = current_state.state_id + 1;

        State { state_id, systems }
    }

    /// Forwards System
    pub fn forward_system(&self, system: &System) -> System {
        let mut next_system: System;
        // Load list of integrators for system.
        let integrators = &system.integrators;
        if integrators.len() > 0 {
            // Apply first integrator to system.
            next_system = self.integrate_system(&integrators[0], &system);
            // Loop over other integrators.
            for integrator in integrators[1..].iter() {
                // Apply possible other integrators to system.
                next_system = self.integrate_system(&integrator, &next_system);
            }
        // handle case of empty integrator-vector
        } else {
            next_system = todo!("Clone vector of integrators");
        }
        next_system
    }

    /// Applies Integration Scheme to a System
    fn integrate_system(&self, integrator: &Integrator, system: &System) -> System {
        let current_state = &self.states[self.config.step_id.1];
        // Load interactions.
        let interactions = &integrator.interactions;
        // Match integration variant to find corresponding integrator function.
        let integrate = match integrator.variant {
            IntegratorVariant::EulerExplicit => euler::explicit::apply,
            _ => todo!("Integrator Variant"),
        };
        integrate(&system, &current_state, &self.states, interactions)
    }

    // pub fn forward_planets(&self, system: &System) -> Planets {
    //     // match system.variant {
    //     // SystemVariant::Planets(_) => {}
    //     // _ => panic!(),
    //     // }

    //     let planets = Planets::new();
    //     let entities = planets
    //         .entities
    //         .iter()
    //         .map(|x| self.forward_planet(x))
    //         .collect();
    //     Planets { entities }
    // }

    // pub fn forward_planet(&self, planet: &Planet) -> Planet {
    //     let mass = todo!();
    //     let pos = todo!();
    //     let vel = todo!();
    //     let res = Planet::new(mass, pos, vel);
    //     res
    // }

    // pub fn send(&self) {}

    // pub fn receive(&self) {}

    pub fn get_unsaved_state_ids(&self) -> Vec<usize> {
        let a = self
            .states
            .iter()
            .filter(|state| {
                state.state_id
                    >= match self.config.last_export_step_id {
                        None => 0,
                        Some(e) => e + 1,
                    }
            })
            .map(|state| state.state_id)
            .collect();
        a
    }
}

// /// Forwards Engine by one time-step
// pub fn step(&mut self) {
//     // /// Forwards State

//     // let mut next_state = State::new();
//     // next_state.state_id = current_state.state_id + 1;
//     // // println!("{:?}", current_state);
//     // /// Creates "neighborhoods"
//     // // let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes
//     // /// Loops over systems & forwards each
//     // for system in &current_state.systems {
//     //     let mut next_system = system.clone();
//     //     /// Gets all Integrators for this System & loops over them
//     //     for integrator in system.integrators.iter() {
//     //         /// Gets all Interacting Systems for this Interaction
//     //         // let other_ids = tmp::get_other_ids(&integrator, &current_state);
//     //         let other_ids: Vec<usize> = todo!();
//     //         /// Applies Interaction
//     //         // integrator.step(&mut next_system, &current_state, &other_ids);
//     //         match integrator {
//     //             _ => todo!(),
//     //         }
//     //     }
//     //     next_state.systems.push(next_system);
//     // }
//     // Forward to next time-step.
//     // let next = current_state.next(&self.config, &self.states);
// }
// // Make sure entities can be used in forwarded system.
// if integrators.len() == 0 {
//     next_system = System {
//         system_id: system.system_id,
//         integrators: system.integrators.clone(),
//         entities: vec![],                   // TODO system.entities.clone(),
//         variant: SystemVariant::EntitiesV1, // TODO system.variant.clone(),
//     };
//     // system.clone();
// } else {

// let mut entities = vec![];
// let entities: Vec<Box<dyn Entity>> = match &system.variant {
// match &system.variant {
//     SystemVariant::EntitiesV1(system) => todo!(),
//     SystemVariant::Field(field_variant) => match field_variant {
//         FieldVariant::FieldVariantV1 => todo!(),
//         FieldVariant::GameOfLife => todo!(),
//         FieldVariant::IsingSpinField => todo!(),
//         _ => todo!(),
//     },
//     SystemVariant::Objects(objects_variant) => match objects_variant {
//         ObjectsVariant::ObjectsVariantV1(objects) => {
//             // let b = objects
//             //     .entities
//             //     .into_iter()
//             //     .map(|x| {
//             //         let a = Box::<dyn Entity>::from(Box::from(x));
//             //         a
//             //     })
//             //     .collect();
//             // b

//             entities.push(objects.entities);
//         }
//         ObjectsVariant::Ants => todo!(),
//         // ObjectsVariant::Planets(planets) => {
//         //     let b = planets
//         //         .entities
//         //         .into_iter()
//         //         .map(|x| Box::<dyn Entity>::from(Box::from(x)))
//         //         .collect();
//         //     b
//         // }
//         // as Vec<Box<dyn Entity>>,
//         ObjectsVariant::Boids => todo!(),
//         ObjectsVariant::ChargedParticles => todo!(),
//         ObjectsVariant::Electrons => todo!(),
//         ObjectsVariant::Neutrons => todo!(),
//         ObjectsVariant::Protons => todo!(),
//         _ => todo!(),
//     },
//     _ => todo!(),
// };

// let entities = &(system.entities());
// let entities: Vec<Box<dyn Entity>> = match &system.variant {
//     SystemVariant::EntitiesV1(system) => todo!(),
//     SystemVariant::Field(field_variant) => match field_variant {
//         FieldVariant::FieldVariantV1 => todo!(),
//         FieldVariant::GameOfLife => todo!(),
//         FieldVariant::IsingSpinField => todo!(),
//         _ => todo!(),
//     },
//     SystemVariant::Objects(objects_variant) => match objects_variant {
//         // ObjectsVariant::ObjectsVariantV1(objects) => objects.entities,
//         ObjectsVariant::Ants => todo!(),
//         // ObjectsVariant::Planets(planets) => &planets.entities,
//         ObjectsVariant::Boids => todo!(),
//         ObjectsVariant::ChargedParticles => todo!(),
//         ObjectsVariant::Electrons => todo!(),
//         ObjectsVariant::Neutrons => todo!(),
//         ObjectsVariant::Protons => todo!(),
//         _ => todo!(),
//     },
//     _ => todo!(),
// };
// for entity in system.
// }

// fn step(entities: &Vec<Box<dyn Entity>>) {}

//     let other_ids = todo!();
//     match integrator.variant {
//         IntegratorVariant::EulerExplicit => crate::integrator::euler_explicit(
//             &mut system,
//             current_state,
//             other_ids,
//             interactions,
//         ),
//         _ => todo!(),
//     }
// }
// }
// next_system

// let next_system = System {
//     integrators: system.integrators.clone(),
//     variant: system.variant.clone(),
//     system_id: system.system_id,
//     entities: vec![],
// };

// Load entities.
// let entities = &system.entities;
// Define next state's vector of systems.
// let mut next_system = System {}
// let mut next_system: System;

// // Loop over "other" systems (including self).
// for other in current_state.systems.iter() {
//     todo!("check whether systems are interacting");
//     // match system.variant {
//     // SystemVariant::Planets(system) => match &other.variant {
//     //     SystemVariant::Planets(other) => {}
//     //     _ => todo!(),
//     // },
//     // _ => todo!(),
//     // }
// }

// let variant = match &system.variant {
//     // SystemVariant::Planets(_) => SystemVariant::Planets(self.forward_planets(system)),
//     _ => todo!(),
// };
// System {
//     system_id,
//     variant,
//     entities,
//     integrators,
// }
