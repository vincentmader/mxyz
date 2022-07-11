#![allow(unused_variables)]
use mxyz_config::EngineConfig;
use mxyz_universe::system::system::System;
// use mxyz_universe::system::SystemVariant;
// use mxyz_universe::entity;
// use mxyz_universe::system::planets::Planets;
// use crate::integrator::Integrator;
// use mxyz_universe::integrator::Integrator;
// use mxyz_universe::integrator::IntegratorVariant;
// use mxyz_universe::interaction::force::Force;
// use mxyz_universe::interaction::force::ForceVariant;
// use mxyz_universe::interaction::Interaction;
// use mxyz_universe::interaction::InteractionVariant;

const NR_OF_STEPS: usize = 10;

pub fn preset(systems: &mut Vec<System>, config: &mut EngineConfig) {
    // I. SYSTEMS
    // ========================================================================
    config.step_id.1 = NR_OF_STEPS;
    // const M: f64 = 1.;
    // const G: f64 = 1.;
    // let r = 2.;

    // System 0: Objects
    // ------------------------------------------------------------------------
    // let system_id = 0;
    // let mut system = Planets::new();
    // let speed = (G * M / r).powf(0.5);
    // for entity_id in 0..2 {
    //     let m = M;
    //     let x = [r * (entity_id as f64 - 0.5), 0., 0.];
    //     let v = [0., speed * (2. * entity_id as f64 - 1.), 0.];
    //     let entity = entity::object::planet::Planet::new(m, x, v);
    //     system.entities.push(entity);
    // }
    // let variant = SystemVariant::Planets(system);

    // II.INTEGRATORS
    // ========================================================================

    // System 0: Objects
    // ------------------------------------------------------------------------
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
    //integrator.interactions = interactions;
    //integrators.push(integrator);

    //let mut system = System::new(system_id, variant);
    //system.integrators = integrators;
    //systems.push(system);
    //
    // config.integrators.push(integrators); // TODO needs to be run for each system!
}
