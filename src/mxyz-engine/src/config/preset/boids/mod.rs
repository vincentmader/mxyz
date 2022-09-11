#![allow(unused_variables)]
#![allow(non_snake_case)]
use crate::config::EngineConfig;
use crate::entity::entity_v1::EntityV1;
use crate::integrator::integrator_variant::object::force::ForceIntegratorVariant;
use crate::integrator::integrator_variant::object::ObjectIntegratorVariant;
use crate::integrator::integrator_variant::IntegratorVariant;
use crate::integrator::Integrator;
use crate::interaction::interaction_variant::force::Force;
use crate::interaction::interaction_variant::force::ForceVariant;
use crate::interaction::interaction_variant::InteractionVariant;
use crate::interaction::Interaction;
use crate::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use crate::system::unsized_system::UnsizedSystem;
use rand::prelude::*;

const NR_OF_STEPS: usize = usize::MAX;
const G: f64 = 1.;
const Q: f64 = 0.;
const M_0: f64 = 1.;
const m1: f64 = 0.1;
const m2: f64 = 0.1;
const r0: f64 = 0.7;
const dr: f64 = 0.05;
const N: usize = 50;

fn setup_systems(systems: &mut Vec<UnsizedSystem>) {
    // SYSTEM 0: STAR
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(0, variant);
    // Entities
    // let central_star = EntityV1::new(M_0, [0., 0., 0.], [0., 0., 0.], Q);
    // system.entities.push(Box::new(central_star));
    systems.push(system);

    let mut rng = rand::thread_rng();

    // SYSTEM 1: PLANETS
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(1, variant);
    // Entities
    let v0 = 0.; //(G * M_0 / r0).powf(0.5);
    for entity_id in 0..N {
        let r: f64 = rng.gen();
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        // let r = r0;
        let x = [r * phi.cos(), r * phi.sin(), 0.];
        let v = [-v0 * phi.sin(), v0 * phi.cos(), 0.];
        let entity = EntityV1::new(m1, x, v, Q);
        system.entities.push(Box::new(entity));
    }
    systems.push(system);
}

fn setup_config(config: &mut EngineConfig, systems: &mut Vec<UnsizedSystem>) {
    config.step_id.1 = NR_OF_STEPS;

    // SYSTEM 0: STAR

    // SYSTEM 1: PLANETS
    let mut integrators = vec![];

    // GRAVITY
    // let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    // let integrator_variant = ObjectIntegratorVariant::ForceIntegrator(integrator_variant);
    // let integrator_variant = IntegratorVariant::Object(integrator_variant);
    // let mut integrator = Integrator::new(integrator_variant);
    // let mut interactions = vec![];
    // let gravity = Force::new(ForceVariant::NewtonianGravity);
    // let interaction_variant = InteractionVariant::Force(gravity);
    // let interaction = Interaction::new(interaction_variant);
    // interactions.push(interaction);
    // integrator.interactions = interactions;
    // integrators.push(integrator); // TODO needs to be run for each system!

    // BOID
    let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    let integrator_variant = ObjectIntegratorVariant::Force(integrator_variant);
    let integrator_variant = IntegratorVariant::Object(integrator_variant);
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    // let force = Force::new(ForceVariant::BoidAttraction);
    // let force = InteractionVariant::Force(force);
    // let force = Interaction::new(force);
    // interactions.push(force);

    // let force = Force::new(ForceVariant::BoidRepulsion);
    // let force = InteractionVariant::Force(force);
    // let force = Interaction::new(force);
    // interactions.push(force);

    // let force = Force::new(ForceVariant::BoidAlignment);
    // let force = InteractionVariant::Force(force);
    // let force = Interaction::new(force);
    // interactions.push(force);

    let force = Force::new(ForceVariant::LennardJones);
    let force = InteractionVariant::Force(force);
    let force = Interaction::new(force);
    interactions.push(force);

    integrator.interactions = interactions;
    integrators.push(integrator); // TODO needs to be run for each system!

    systems[1].integrators = integrators;
}

pub fn preset(systems: &mut Vec<UnsizedSystem>, config: &mut EngineConfig) {
    setup_systems(systems);
    setup_config(config, systems);
}
