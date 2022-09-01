#![allow(unused_variables)]
#![allow(non_snake_case)]
use crate::config::EngineConfig;
use crate::entity::entity_v1::EntityV1;
use crate::integrator::ForceIntegratorVariant;
use crate::integrator::Integrator;
use crate::integrator::IntegratorVariant;
use crate::interaction::force::Force;
use crate::interaction::force::ForceVariant;
use crate::interaction::Interaction;
use crate::interaction::InteractionVariant;
use crate::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use crate::system::unsized_system::UnsizedSystem;
use rand::prelude::*;

const NR_OF_STEPS: usize = 10;
const G: f64 = 1.;
const Q: f64 = 1.;
const N: usize = 50;
const M_P: f64 = 1.;
const M_E: f64 = 0.1;

pub fn preset(systems: &mut Vec<UnsizedSystem>, config: &mut EngineConfig) {
    config.step_id.1 = NR_OF_STEPS;

    let mut rng = rand::thread_rng();

    // System 1: PROTONS
    // ------------------------------------------------------------------------
    // SYSTEM
    let system_id = 0;
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(system_id, variant);
    for entity_id in 0..N {
        let x: [f64; 3] = [rng.gen(), rng.gen(), rng.gen()];
        let x = [x[0] * 2. - 1., x[1] * 2. - 1., x[2] * 2. - 1.];
        let v = [0., 0., 0.];
        let entity = EntityV1::new(M_P, x, v, Q);
        system.entities.push(Box::new(entity));
    }
    // INTEGRATORS
    let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    let integrator_variant = IntegratorVariant::ForceIntegratorV1(integrator_variant);
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    let force_variant = ForceVariant::Coulomb;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let mut interaction = Interaction::new(interaction_variant);
    interaction.matrix.init(&systems);
    interaction.matrix.entries.push(Some(true));
    interaction.matrix.entries.push(Some(true));
    interactions.push(interaction);
    integrator.interactions = interactions;
    system.integrators.push(integrator); // TODO needs to be run for each system!
    systems.push(system);

    // System 1: ELECTRONS
    // ------------------------------------------------------------------------
    // SYSTEM
    let system_id = 1;
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(system_id, variant);
    for entity_id in 0..N {
        let x = [rng.gen(), rng.gen(), rng.gen()];
        let v = [0., 0., 0.];
        let entity = EntityV1::new(M_E, x, v, Q);
        system.entities.push(Box::new(entity));
    }
    // INTEGRATORS
    let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    let integrator_variant = IntegratorVariant::ForceIntegratorV1(integrator_variant);
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    let force_variant = ForceVariant::Coulomb;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let mut interaction = Interaction::new(interaction_variant);
    interaction.matrix.init(&systems);
    interaction.matrix.entries.push(Some(true));
    interaction.matrix.entries.push(Some(true));
    interactions.push(interaction);
    integrator.interactions = interactions;
    system.integrators.push(integrator); // TODO needs to be run for each system!
    systems.push(system);
}
