use super::interaction::Interaction;
// use crate::state::State;
use mxyz_universe::entity::attribute::Mass;
use mxyz_universe::entity::attribute::Position;
use mxyz_universe::entity::attribute::Velocity;
use mxyz_universe::integrator::IntegratorVariant;
use mxyz_universe::interaction::InteractionVariant;
use mxyz_universe::state::State;
use mxyz_universe::system::System;
use mxyz_universe::system::SystemVariant;

const DT: f64 = 0.1; // TODO move else-where

#[derive(Debug)]
pub struct Integrator {
    pub variant: IntegratorVariant,
    pub interactions: Vec<Interaction>,
    //  TODO specify neighborhood/tree calculation (or in interaction?)
}
impl Integrator {
    pub fn new(variant: IntegratorVariant) -> Self {
        let interactions = vec![];
        Integrator {
            variant,
            interactions,
        }
    }
    pub fn step(&self, system: &mut System, state: &State, other_ids: &Vec<usize>) {
        // ! println!("\t{:#?}: {:?}", self.variant, other_ids);
        let stepper = match self.variant {
            IntegratorVariant::EulerExplicit => euler_explicit,
            IntegratorVariant::CellularAutomaton => cellular_automaton,
            IntegratorVariant::Collision => collision,
            _ => todo!(),
        };
        stepper(system, state, other_ids, &self.interactions);
    }
}

// pub fn euler_explicit(
//     entity: &mut Box<dyn PhysicalObject>,
//     other: &Box<dyn PhysicalObject>,
//     force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
// ) {
//     let f = force_getter(entity, &other);
//     let m1 = entity.get_mass();
//     let a = [f[0] / m1, f[1] / m1, f[2] / m1];
//     const DT: f64 = 0.001; // TODO
//     let v1 = entity.get_velocity();
//     let v1: Vec<f64> = (0..3).map(|i| v1[i] + a[i] * DT).collect();
//     entity.set_velocity(&[v1[0], v1[1], v1[2]]);
//     let y1 = entity.get_position();
//     let y1: Vec<f64> = (0..3).map(|i| y1[i] + v1[i] * DT).collect();
//     entity.set_position(&[y1[0], y1[1], y1[2]]);
// }

// pub fn runge_kutta_4(
//     _entity: &mut Box<dyn PhysicalObject>,
//     _other: &Box<dyn PhysicalObject>,
//     _force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
// ) {
// }

// pub fn verlet(
//     _entity: &mut Box<dyn PhysicalObject>,
//     _other: &Box<dyn PhysicalObject>,
//     _force_getter: fn(&Box<dyn PhysicalObject>, &Box<dyn PhysicalObject>) -> [f64; 3],
// ) {
// }

/// Explicit Euler:
/// - dy/dt = a(t,y) =  f(t,y)
/// Steps:
/// - get f(t,y) as sum of f(t,y) for all interacting entities
/// - update velocity using f(t,y)
fn euler_explicit(
    system: &mut System,
    state: &State,
    other_ids: &Vec<usize>,
    interactions: &Vec<Interaction>,
) {
    let system_id = system.system_id;

    // let entities = match &mut system.variant{}

    match &mut system.variant {
        SystemVariant::PhysicalObjects(_system) => {}
        SystemVariant::Planets(system) => {
            let entity_ids = 0..system.entities.len(); // TODO only update some?
            for entity_id in entity_ids {
                // ! println!("\t\tENT-{}", entity_id);
                let entity = &mut system.entities[entity_id];

                let mut acceleration = [0., 0., 0.];
                /// Loops over the other systems
                for other_sys_id in other_ids.iter() {
                    let other = &state.systems.get(*other_sys_id).unwrap();
                    let other_entities = match &other.variant {
                        SystemVariant::Planets(system) => &system.entities, // TODO get from neighborhood
                        _ => todo!(),
                    };
                    /// Loops over the Integrator's Interactions (skips if it doesn't apply)
                    //  TODO get interactions to-apply outside of entity-loop (?)
                    // ! println!("\t\t\tOTHER-{}", other_sys_id);
                    for interaction in interactions.iter() {
                        if interaction.matrix.entries[*other_sys_id].unwrap() == false {
                            continue;
                        }
                        // ! println!("\t\t\t\t{:?}", interaction.variant);
                        /// Loops over the Entities in the interacting System
                        let other_ids = 0..other_entities.len(); // TODO get ids
                        for other_id in other_ids {
                            let foo = (system_id, entity_id) == (*other_sys_id, other_id);
                            if foo {
                                continue; // TODO not working?
                            }
                            let other = &other_entities[other_id];
                            /// Updates Velocity
                            match &interaction.variant {
                                InteractionVariant::Force(f) => {
                                    let mass_1 = entity.get_mass(); // TODO move further up?
                                    let force = f.calculate_from(entity, other);
                                    // ! println!(
                                    // !     "\t\t\t\t\tENT-{}, pos:\t{:?}",
                                    // !     entity_id,
                                    // !     entity.get_position()
                                    // ! );
                                    // ! println!(
                                    // !     "\t\t\t\t\t\t{:?} <-> {:?}",
                                    // !     (system_id, entity_id),
                                    // !     (other_sys_id, other_id)
                                    // ! );
                                    // ! println!("\t\t\t\t\t\t-> F = {:?}", force);
                                    acceleration = [
                                        acceleration[0] + force[0] / mass_1 * DT,
                                        acceleration[1] + force[1] / mass_1 * DT,
                                        acceleration[2] + force[2] / mass_1 * DT,
                                    ];
                                }
                                _ => todo!(),
                            }
                        }
                    }
                }
                /// Updates Velocity Vector
                let velocity = entity.get_velocity();
                let velocity = [
                    velocity[0] + acceleration[0] * DT,
                    velocity[1] + acceleration[1] * DT,
                    velocity[2] + acceleration[2] * DT,
                ];
                entity.set_velocity(&velocity);
                /// Updates Position Vector
                let position = entity.get_position();
                let position = [
                    position[0] + velocity[0] * DT,
                    position[1] + velocity[1] * DT,
                    position[2] + velocity[2] * DT,
                ];
                entity.set_position(&position);
            }
        }
        _ => todo!(),
    }
}

pub fn cellular_automaton(
    _system: &mut System,
    _state: &State,
    _other_ids: &Vec<usize>,
    _interactions: &Vec<Interaction>,
) {
}

pub fn collision(
    _system: &mut System,
    _state: &State,
    _other_ids: &Vec<usize>,
    _interactions: &Vec<Interaction>,
) {
}
