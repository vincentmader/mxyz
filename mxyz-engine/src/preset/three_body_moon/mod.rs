#![allow(unused_variables)]
use mxyz_config::EngineConfig;
use mxyz_universe::entity::EntityV1;
use mxyz_universe::integrator::Integrator;
use mxyz_universe::integrator::IntegratorVariant;
use mxyz_universe::interaction::force::Force;
use mxyz_universe::interaction::force::ForceVariant;
use mxyz_universe::interaction::Interaction;
use mxyz_universe::interaction::InteractionVariant;
use mxyz_universe::system::system::System;
use mxyz_universe::system::system::SystemVariant;
// use mxyz_universe::system::SystemVariant;
// use mxyz_universe::entity;
// use mxyz_universe::system::planets::Planets;
// use crate::integrator::Integrator;

const NR_OF_STEPS: usize = 10;
const G: f64 = 1.;

pub fn preset(systems: &mut Vec<System>, config: &mut EngineConfig) {
    // I. SYSTEMS
    // ========================================================================
    config.step_id.1 = NR_OF_STEPS;

    let m = 1.;
    let dist = 1.;
    let M = 1000.;
    let speed = (G * M / dist).powf(0.5) * 0.5;
    let N = 1;
    let r0 = 0.8;
    let v0 = speed;

    // System 0: Objects
    // ------------------------------------------------------------------------
    let system_id = 0;
    let variant = SystemVariant::EntitiesV1;
    let mut system = System::new(system_id, variant);
    for entity_id in 0..N {
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        let x = [r0 * phi.cos(), r0 * phi.sin(), 0.];
        let v = [-v0 * phi.sin(), v0 * phi.cos(), 0.];
        let entity = EntityV1::new(m, x, v);
        system.entities.push(Box::new(entity));
    }

    let entity = EntityV1::new(M, [0., 0., 0.], [0., 0., 0.]);
    system.entities.push(Box::new(entity));

    // System 0: Objects
    // ------------------------------------------------------------------------
    // let system_id = 0;
    // let variant = SystemVariant::EntitiesV1;
    // let mut system = System::new(system_id, variant);
    // for entity_id in 0..2 {
    //     let x = [dist * (entity_id as f64 - 0.5), 0., 0.];
    //     let v = [0., speed * (2. * entity_id as f64 - 1.), 0.];
    //     // let v = [0., 0., 0.];
    //     let entity = EntityV1::new(m, x, v);
    //     system.entities.push(Box::new(entity));
    // }

    // II.INTEGRATORS
    // ========================================================================

    // System 0: Objects
    // ------------------------------------------------------------------------

    let integrator_variant = IntegratorVariant::EulerExplicit;
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    let force_variant = ForceVariant::NewtonianGravity;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let mut interaction = Interaction::new(interaction_variant);
    interaction.matrix.init(&systems);
    interaction.matrix.entries[0] = Some(true);
    interactions.push(interaction);

    integrator.interactions = interactions;

    system.integrators.push(integrator); // TODO needs to be run for each system!
    systems.push(system);
}
