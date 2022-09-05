#![allow(unreachable_code)]
use crate::config::preset;
use crate::config::simulation_variant::SimulationVariant;
use crate::config::EngineConfig;
use crate::entity::Entity;
use crate::integrator::integrator_variant::object::force::ForceIntegratorVariant;
use crate::integrator::integrator_variant::IntegratorVariant;
use crate::integrator::Integrator;
use crate::interaction::force::ForceVariant;
use crate::interaction::Interaction;
use crate::interaction::InteractionVariant;
use crate::state::UnsizedState;
use crate::system::UnsizedSystem;

const DT: f64 = 0.01;

/// MXYZ Simulation-Engine
pub trait Engine {
    fn engine_id(&self) -> &usize;
    fn engine_states(&self) -> &Vec<State>;
    fn engine_states_mut(&mut self) -> &mut Vec<State>;
    fn engine_config(&self) -> &EngineConfig;
    fn engine_config_mut(&mut self) -> &mut EngineConfig;
    fn add_engine_state(&mut self, state: State) {
        self.engine_states_mut().push(state);
    }
    /// Initialize state & engine-config.
    fn init(&mut self, sim_variant: Option<SimulationVariant>) {
        let initial_state = preset::initialize(sim_variant, &mut self.engine_config_mut());
        self.add_engine_state(initial_state);
    }
    /// Run engine.
    fn run(&mut self) {
        // Run compute-loop for nr. of steps specified in engine configuration.
        let max_state_id = self.engine_config().step_id.1;
        for _state_id in 0..max_state_id {
            // Forward engine to next time-step.
            self.forward_engine();
        }
    }
    /// Forward Engine to next time-step.
    fn forward_engine(&mut self) {
        // Load current state.
        let state_id = self.engine_config().step_id.0;
        let current_state = &self.engine_states()[state_id];
        // Build trees / neighborhoods
        // TODO ...
        // Forward state to next time-step.
        let next_state = self.forward_state(&current_state);
        // Append new state to state-vector.
        self.add_engine_state(next_state);
        // Update step-id in engine-config.
        self.engine_config_mut().step_id.0 += 1;
    }
    /// Forward state to next time-step.
    fn forward_state(&self, state: &State) -> State;
    /// Forward system to next time-step.
    fn forward_system(&self, system: (usize, &System)) -> System {
        let (system_id, system) = system;
        // Load list of integrators for system.
        let integrators = &system.integrators;
        // Loop over all integrators.
        let next_system = match integrators.len() {
            0 => system.clone(),
            _ => {
                let mut next_system = self.integrate_system(&integrators[0], (system_id, &system));
                // Apply possible other integrators to system.
                for integrator in integrators[1..].iter() {
                    next_system = self.integrate_system(integrator, (system_id, &system));
                }
                next_system
            }
        };
        next_system
    }
    /// Apply integration scheme to system.
    fn integrate_system(&self, integrator: &Integrator, system: (usize, &System)) -> System;
    /// Apply integration scheme to entity.
    fn integrate_entity(
        &self,
        integrator: &Integrator,
        entity: ((usize, usize), &Box<dyn Entity>),
    ) -> Box<dyn Entity> {
        let (entity_id, entity) = entity;
        // Get current state.
        let state_id = self.engine_states().len() - 1;
        let state = self.engine_states().get(state_id).unwrap();
        let interactions = &integrator.interactions;
        // ...
        let (m, pos, vel) = (
            entity.get_mass(),
            entity.get_position(),
            entity.get_velocity(),
        );
        match &integrator.variant {
            IntegratorVariant::ForceIntegratorV1(integrator) => match integrator {
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
                    let q = 1.;
                    let entity = crate::entity::entity_v1::EntityV1::new(m, pos, vel, q);
                    Box::new(entity)
                }
                _ => todo!("Force Integrator Variant"),
            },
            _ => todo!("Integrator Variant"),
        }
    }
}

// InteractionVariant::Force(f) => {
//     let force = match f.variant {
//         ForceVariant::NewtonianGravity => {
//             const G: f64 = 1.;
//             let (m1, m2) = (entity.get_mass(), other.get_mass());
//             let conn: Vec<f64> = (0..3)
//                 .map(|i| {
//                     entity.get_position()[i]
//                         - other.get_position()[i]
//                 })
//                 .collect();
//             let r = conn
//                 .iter()
//                 .map(|x| x.powf(2.))
//                 .sum::<f64>()
//                 .powf(0.5);
//             let unit: Vec<f64> =
//                 conn.iter().map(|x| x / r).collect();
//             let force = G * m1 * m2 / r.powf(2.);
//             [force * unit[0], force * unit[1], force * unit[2]]
//         }
//         _ => todo!("other forces"),
//     };
//     let acc = [force[0] / m, force[1] / m, force[2] / m];
//     v = [
//         v[0] + acc[0] * DT,
//         v[1] + acc[1] * DT,
//         v[2] + acc[2] * DT,
//     ];
//     p = [p[0] + v[0] * DT, p[1] + v[1] * DT, p[2] + v[2] * DT];
//     println!("{:?}", p)
// }
// _ => todo!("other interactions"),
