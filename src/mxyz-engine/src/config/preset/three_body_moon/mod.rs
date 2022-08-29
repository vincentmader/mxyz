#![allow(unused_variables)]
use crate::config::EngineConfig;
use mxyz_engine_universe::entity::EntityV1;
use mxyz_engine_universe::integrator::Integrator;
use mxyz_engine_universe::integrator::IntegratorVariant;
use mxyz_engine_universe::interaction::force::Force;
use mxyz_engine_universe::interaction::force::ForceVariant;
use mxyz_engine_universe::interaction::Interaction;
use mxyz_engine_universe::interaction::InteractionVariant;
use mxyz_engine_universe::system::System;
use mxyz_engine_universe::system::SystemVariant;

const NR_OF_STEPS: usize = 10;
const G: f64 = 1.;

pub fn preset(systems: &mut Vec<System>, config: &mut EngineConfig) {
    config.step_id.1 = NR_OF_STEPS;

    let m = 1.;
    let M = 1.;
    let r0 = 0.7;
    let v0 = (G * M / r0).powf(0.5);
    let N = 4;

    // System 0: STAR
    // ------------------------------------------------------------------------
    // SYSTEM
    let system_id = 0;
    let variant = SystemVariant::EntitiesV1;
    let mut system = System::new(system_id, variant);
    let entity = EntityV1::new(M, [0., 0., 0.], [0., 0., 0.]);
    system.entities.push(Box::new(entity));
    systems.push(system);

    // System 1: PLANETS
    // ------------------------------------------------------------------------
    // SYSTEM
    let system_id = 1;
    let variant = SystemVariant::EntitiesV1;
    let mut system = System::new(system_id, variant);
    for entity_id in 0..N {
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        let x = [r0 * phi.cos(), r0 * phi.sin(), 0.];
        let v = [-v0 * phi.sin(), v0 * phi.cos(), 0.];
        let entity = EntityV1::new(m, x, v);
        system.entities.push(Box::new(entity));
    }
    // INTEGRATORS
    let integrator_variant = IntegratorVariant::EulerExplicit;
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    let force_variant = ForceVariant::NewtonianGravity;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let mut interaction = Interaction::new(interaction_variant);
    interaction.matrix.init(&systems);
    interaction.matrix.entries[0] = Some(true);
    interaction.matrix.entries[1] = Some(true);
    interactions.push(interaction);
    integrator.interactions = interactions;
    system.integrators.push(integrator); // TODO needs to be run for each system!
    systems.push(system);

    // System 1: PLANETS
    // ------------------------------------------------------------------------
    let dr = 0.1;
    let r = r0 + dr;
    let v0 = v0 + (G * m / dr).powf(0.5);
    let mu = 0.001;
    // SYSTEM
    let system_id = 2;
    let variant = SystemVariant::EntitiesV1;
    let mut system = System::new(system_id, variant);
    for entity_id in 0..N {
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        // let phi = phi + 3.14159;
        let x = [r * phi.cos(), r * phi.sin(), 0.];
        let v = [-v0 * phi.sin(), v0 * phi.cos(), 0.];
        let entity = EntityV1::new(mu, x, v);
        system.entities.push(Box::new(entity));
    }
    // INTEGRATORS
    let integrator_variant = IntegratorVariant::EulerExplicit;
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    let force_variant = ForceVariant::NewtonianGravity;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let mut interaction = Interaction::new(interaction_variant);
    interaction.matrix.init(&systems);
    interaction.matrix.entries[0] = Some(true);
    interaction.matrix.entries[1] = Some(true);
    interaction.matrix.entries[2] = Some(true);
    interactions.push(interaction);
    integrator.interactions = interactions;
    system.integrators.push(integrator); // TODO needs to be run for each system!
    systems.push(system);
}
