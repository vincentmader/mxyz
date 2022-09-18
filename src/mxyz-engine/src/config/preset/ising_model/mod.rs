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
use crate::neighborhoods::NeighborhoodVariant;
use crate::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use crate::system::unsized_system::UnsizedSystem;

// const NR_OF_STEPS: u128 = usize::MAX;
const G: f64 = 1.;
const Q: f64 = 0.;
const M_0: f64 = 1.;
const M_1: f64 = 0.1;
const M_2: f64 = 0.00001;
const R_0: f64 = 0.7;
const DR: f64 = 0.05;
const N: usize = 70;

fn setup_systems(systems: &mut Vec<UnsizedSystem>) {
    // SYSTEM 0: PLANETS
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(0, variant);
    // Entities
    let v_0 = (G * M_0 / R_0).powf(0.5) / 5.;
    for entity_id in 0..N {
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        let x = [R_0 * phi.cos(), R_0 * phi.sin(), 0.];
        let v = [-v_0 * phi.sin(), v_0 * phi.cos(), 0.];
        let entity = EntityV1::new(M_1, x, v, Q);
        system.entities.push(Box::new(entity));
    }
    systems.push(system);
}

fn setup_config(config: &mut EngineConfig, systems: &mut Vec<UnsizedSystem>) {
    let nr_of_systems = systems.len();

    // SYSTEM 0: PLANETS
    let mut integrators = vec![];

    let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    let integrator_variant = ObjectIntegratorVariant::Force(integrator_variant);
    let integrator_variant = IntegratorVariant::Object(integrator_variant);

    let mut integrator = Integrator::new(integrator_variant, nr_of_systems);
    let mut interactions = vec![];

    let force_variant = ForceVariant::NewtonianGravity;
    let force = Force::new(force_variant);
    let interaction_variant = InteractionVariant::Force(force);
    let interaction = Interaction::new(interaction_variant);
    interactions.push(interaction);

    integrator.interactions = interactions;

    let neighborhood = NeighborhoodVariant::All;
    integrator.matrix.0[0][0] = neighborhood;

    integrators.push(integrator); // NOTE needs to be run for each system!
    systems[0].integrators = integrators;
}

pub fn preset(systems: &mut Vec<UnsizedSystem>, config: &mut EngineConfig) {
    setup_systems(systems);
    setup_config(config, systems);
}
