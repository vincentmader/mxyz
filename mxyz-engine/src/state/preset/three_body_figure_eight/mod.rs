use crate::config::EngineConfig;
use mxyz_universe::system::System;
// use crate::entity;
// use crate::integrator::{Integrator, IntegratorVariant};
// use crate::interaction::collision::Collision;
// use crate::interaction::force::{Force, ForceVariant};
// use crate::interaction::{Interaction, InteractionVariant};
// use crate::system::SystemVariant;

// const NR_OF_STEPS: usize = 2221;
const NR_OF_STEPS: usize = 10;

pub fn preset(_systems: &mut Vec<System>, config: &mut EngineConfig) {
    //// I. SYSTEMS
    //// ========================================================================
    config.step_id.1 = NR_OF_STEPS;

    //// System 0: Objects
    //// ------------------------------------------------------------------------
    //let variant = SystemVariant::PhysicalObjects;
    //let mut system = System::new(variant);
    //let speed = 0.;
    //for entity_id in 0..2 {
    //    let m = 1.;
    //    let x = [2. * (entity_id as f64 - 0.5), 0., 0.];
    //    let v = [0., speed * (2. * entity_id as f64 - 1.), 0.];
    //    let entity = entity::object::planet::Planet::new(m, x, v);
    //    system.entities.push(Box::new(entity));
    //}
    //systems.push(system);

    //// System 1: Field
    //// ------------------------------------------------------------------------
    //let variant = SystemVariant::DiscreteField;
    //let mut system = System::new(variant);
    //for _ in 0..2 {
    //    let vel = [0., 0., 0.];
    //    let dens = 1.;
    //    let entity = entity::field::fluid_cell::FluidCell::new(vel, dens);
    //    system.entities.push(Box::new(entity));
    //}
    //systems.push(system);

    //// III.INTEGRATORS
    //// ========================================================================

    //// System 0: Objects
    //// ------------------------------------------------------------------------
    //let mut integrators = vec![];
    ////
    //let mut integrator = Integrator::new(IntegratorVariant::EulerExplicit);
    //let mut interactions = vec![];
    ////
    //let force = Force::new(ForceVariant::NewtonianGravity);
    //let mut interaction = Interaction::new(InteractionVariant::Force(force));
    //interaction.matrix.init(&systems);
    //interaction.matrix.entries[0] = Some(true);
    //interactions.push(interaction);
    ////
    //let force = Force::new(ForceVariant::Coulomb);
    //let mut interaction = Interaction::new(InteractionVariant::Force(force));
    //interaction.matrix.init(&systems);
    //interaction.matrix.entries[0] = Some(true);
    //interactions.push(interaction);
    ////
    //integrator.interactions = interactions;
    //integrators.push(integrator);
    ////
    //let mut integrator = Integrator::new(IntegratorVariant::Collision);
    //let mut interactions = vec![];
    ////
    //let coll = Collision::new();
    //let mut interaction = Interaction::new(InteractionVariant::Collision(coll));
    //interaction.matrix.init(&systems);
    //interaction.matrix.entries[0] = Some(true);
    //interactions.push(interaction);
    //integrator.interactions = interactions;
    //integrators.push(integrator);
    ////
    //config.integrators.push(integrators); // TODO needs to be run for each system!

    //// System 1: Field
    //// ------------------------------------------------------------------------
    //let mut integrators = vec![];
    //let mut integrator = Integrator::new(IntegratorVariant::CellularAutomaton);
    //let mut interactions = vec![];
    //let force = Force::new(ForceVariant::NewtonianGravity);
    //let mut interaction = Interaction::new(InteractionVariant::Force(force));
    //interaction.matrix.init(&systems);
    //interaction.matrix.entries[0] = Some(true);
    //interactions.push(interaction);
    //integrator.interactions = interactions;
    //integrators.push(integrator);
    //config.integrators.push(integrators);

    //println!("\n\n{}", systems[0].entities.len());
}
