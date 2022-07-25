use crate::config::EngineConfig;
use crate::integrator::{Integrator, IntegratorVariant};
use crate::interaction::force::{Force, ForceVariant};
use crate::interaction::{Interaction, InteractionVariant};
use mxyz_universe::entity;
use mxyz_universe::system::planets::Planets;
use mxyz_universe::system::{System, SystemVariant};

const NR_OF_STEPS: usize = 10;

pub fn preset(systems: &mut Vec<System>, config: &mut EngineConfig) {
    // I. SYSTEMS
    // ========================================================================
    config.step_id.1 = NR_OF_STEPS;

    // System 0: Objects
    // ------------------------------------------------------------------------
    let system_id = 0;
    let mut system = Planets::new();
    let speed = 0.;
    for entity_id in 0..2 {
        let m = 1.;
        let x = [2. * (entity_id as f64 - 0.5), 0., 0.];
        let v = [0., speed * (2. * entity_id as f64 - 1.), 0.];
        let entity = entity::object::planet::Planet::new(m, x, v);
        system.entities.push(entity);
    }
    let variant = SystemVariant::Planets(system);
    let system = System::new(system_id, variant);
    systems.push(system);

    // III.INTEGRATORS
    // ========================================================================

    // System 0: Objects
    // ------------------------------------------------------------------------
    let mut integrators = vec![];
    //
    let mut integrator = Integrator::new(IntegratorVariant::EulerExplicit);
    let mut interactions = vec![];
    //
    let force = Force::new(ForceVariant::NewtonianGravity);
    let mut interaction = Interaction::new(InteractionVariant::Force(force));
    interaction.matrix.init(&systems);
    interaction.matrix.entries[0] = Some(true);
    interactions.push(interaction);
    //
    integrator.interactions = interactions;
    integrators.push(integrator);
    //
    config.integrators.push(integrators); // TODO needs to be run for each system!
}
