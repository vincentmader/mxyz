#![allow(unused_variables)]
#![allow(non_snake_case)]
use crate::config::EngineConfig;
use crate::entity::entity_v1::EntityV1;
use crate::integrator::integrator_variant::object::force::ForceIntegratorVariant;
use crate::integrator::integrator_variant::object::ObjectIntegratorVariant;
use crate::integrator::integrator_variant::IntegratorVariant;
use crate::integrator::Integrator;
use crate::interaction::force::Force;
use crate::interaction::force::ForceVariant;
use crate::interaction::Interaction;
use crate::interaction::InteractionVariant;
use crate::system::unsized_system::unsized_system_variant::UnsizedSystemVariant;
use crate::system::unsized_system::UnsizedSystem;

const NR_OF_STEPS: usize = usize::MAX;
const G: f64 = 1.;
const Q: f64 = 0.;
const M_0: f64 = 1.;
const m1: f64 = 0.1;
const m2: f64 = 0.00001;
const r0: f64 = 0.7;
const dr: f64 = 0.05;
const N: usize = 5;

fn setup_systems(systems: &mut Vec<UnsizedSystem>) {
    // SYSTEM 0: STAR
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(0, variant);
    // Entities
    let central_star = EntityV1::new(M_0, [0., 0., 0.], [0., 0., 0.], Q);
    system.entities.push(Box::new(central_star));
    systems.push(system);

    // SYSTEM 1: PLANETS
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(1, variant);
    // Entities
    let v0 = (G * M_0 / r0).powf(0.5);
    for entity_id in 0..N {
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        let x = [r0 * phi.cos(), r0 * phi.sin(), 0.];
        let v = [-v0 * phi.sin(), v0 * phi.cos(), 0.];
        let entity = EntityV1::new(m1, x, v, Q);
        system.entities.push(Box::new(entity));
    }
    systems.push(system);

    // SYSTEM 2: MOONS
    let variant = UnsizedSystemVariant::EntitiesV1;
    let mut system = UnsizedSystem::new(2, variant);
    for entity_id in 0..N {
        let phi = 2. * 3.14159 * entity_id as f64 / N as f64;
        let x = [r0 * phi.cos(), r0 * phi.sin(), 0.];
        let v_K = (G * m1 / dr).powf(0.5);
        let v = [-v0 * phi.sin(), v0 * phi.cos(), 0.];
        for moon_id in 0..N {
            let theta = 2. * 3.14159 * moon_id as f64 / N as f64;
            let x = [x[0] + dr * theta.cos(), x[1] + dr * theta.sin(), 0.];
            let v = [v[0] - v_K * theta.sin(), v[1] + v_K * theta.cos(), 0.];
            let entity = EntityV1::new(m2, x, v, Q);
            system.entities.push(Box::new(entity));
        }
    }
    systems.push(system);
}

fn setup_config(config: &mut EngineConfig) {
    config.step_id.1 = NR_OF_STEPS;

    // SYSTEM 0: STAR
    let system_cfg = crate::config::SystemConfig::new();
    config.systems.insert(0, system_cfg);

    // SYSTEM 1: PLANETS
    let mut integrators = vec![];

    let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    let integrator_variant = ObjectIntegratorVariant::ForceIntegrator(integrator_variant);
    let integrator_variant = IntegratorVariant::Object(integrator_variant);
    let mut integrator = Integrator::new(integrator_variant);

    let mut interactions = vec![];

    let force_variant = ForceVariant::NewtonianGravity;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let mut interaction = Interaction::new(interaction_variant);
    interactions.push(interaction);
    integrator.interactions = interactions;
    integrators.push(integrator); // TODO needs to be run for each system!
    let mut system_cfg = crate::config::SystemConfig::new();
    system_cfg.integrators = integrators;
    config.systems.insert(1, system_cfg);

    // SYSTEM 2: MOONS
    let mut integrators = vec![];
    let integrator_variant = ForceIntegratorVariant::EulerExplicit;
    let integrator_variant = ObjectIntegratorVariant::ForceIntegrator(integrator_variant);
    let integrator_variant = IntegratorVariant::Object(integrator_variant);
    let mut integrator = Integrator::new(integrator_variant);
    let mut interactions = vec![];

    let force_variant = ForceVariant::NewtonianGravity;
    let force = Force::new(force_variant);

    let interaction_variant = InteractionVariant::Force(force);
    let interaction = Interaction::new(interaction_variant);
    interactions.push(interaction);
    integrator.interactions = interactions;
    integrators.push(integrator);
    let mut system_cfg = crate::config::SystemConfig::new();
    system_cfg.integrators = integrators;
    config.systems.insert(2, system_cfg);
}

pub fn preset(systems: &mut Vec<UnsizedSystem>, config: &mut EngineConfig) {
    setup_systems(systems);
    setup_config(config);
}
