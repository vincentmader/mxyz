#![allow(unreachable_code)]
use mxyz_config::EngineConfig;
use mxyz_universe::entity::object::planet::Planet;
use mxyz_universe::preset::SimulationVariant;
use mxyz_universe::state::State;
use mxyz_universe::system::objects::planets::Planets;
use mxyz_universe::system::System;

// ============================================================================

/// MXYZ Simulation Engine
pub struct Engine {
    pub engine_id: usize,
    pub config: EngineConfig,
    pub states: Vec<State>,
}
impl Engine {
    /// Creates a new Engine instance
    pub fn new(engine_id: usize) -> Self {
        let config = EngineConfig::new();
        let states = vec![];
        Engine {
            engine_id,
            config,
            states,
        }
    }

    /// Gets Initial State & sets up Configuration for a given Simulation Variant
    pub fn initial_state(
        simulation_variant: Option<SimulationVariant>,
        cfg: &mut EngineConfig,
    ) -> State {
        match simulation_variant {
            Some(simulation_variant) => match simulation_variant {
                SimulationVariant::ThreeBodyMoon => {
                    let mut state = State::new();
                    let mut systems = vec![];
                    crate::preset::three_body_moon::preset(&mut systems, cfg);
                    state.systems = systems;
                    state
                }
                _ => todo!(),
            },
            None => todo!("handle this earlier? (in str->enum sim-var conversion)"),
        }
    }

    /// Initializes State & Config
    pub fn init(&mut self, simulation_variant: Option<SimulationVariant>) {
        println!("MXYZ-Engine: Initializing...");
        let initial_state = Self::initial_state(simulation_variant, &mut self.config);
        println!("{:?}", initial_state);
        self.states.push(initial_state);
    }

    /// Runs Engine
    pub fn run(&mut self) {
        println!("MXYZ-Engine: Running...");
        for _ in 0..self.config.step_id.1 {
            self.step();
        }
    }

    /// Forwards Engine by one time-step
    pub fn step(&mut self) {
        /// Forwards State
        let next_state = self.forward_state();

        // let mut next_state = State::new();
        // next_state.state_id = current_state.state_id + 1;
        // // println!("{:?}", current_state);
        // /// Creates "neighborhoods"
        // // let _neighborhoods = tmp::prepare_neighborhoods(); // TODO get relevant neighbors/nodes
        // /// Loops over systems & forwards each
        // for system in &current_state.systems {
        //     let mut next_system = system.clone();
        //     /// Gets all Integrators for this System & loops over them
        //     for integrator in system.integrators.iter() {
        //         /// Gets all Interacting Systems for this Interaction
        //         // let other_ids = tmp::get_other_ids(&integrator, &current_state);
        //         let other_ids: Vec<usize> = todo!();
        //         /// Applies Interaction
        //         // integrator.step(&mut next_system, &current_state, &other_ids);
        //         match integrator {
        //             _ => todo!(),
        //         }
        //     }
        //     next_state.systems.push(next_system);
        // }
        // Forward to next time-step.
        // let next = current_state.next(&self.config, &self.states);

        self.states.push(next_state);
        self.config.step_id.0 += 1;
    }

    /// Forwards State
    pub fn forward_state(&self) -> State {
        let current_state = &self.states[self.config.step_id.0];
        let state_id = current_state.state_id + 1;
        let systems = current_state
            .systems
            .iter()
            .map(|sys| self.forward_system(sys))
            .collect();

        State { state_id, systems }
    }

    /// Forwards System
    pub fn forward_system(&self, system: &System) -> System {
        // Load current State.
        let current_state = &self.states[self.config.step_id.0];
        // Set System-ID & Integrators for next State.
        let system_id = system.system_id;
        let integrators = system.integrators.clone(); // todo better way?

        // for integrator in integrators.iter() {
        //     let interactions = &integrator.interactions;
        //     let other_ids = todo!();
        //     match integrator.variant {
        //         IntegratorVariant::EulerExplicit => crate::integrator::euler_explicit(
        //             &mut system,
        //             current_state,
        //             other_ids,
        //             interactions,
        //         ),
        //         _ => todo!(),
        //     }
        // }

        // Loop over "other" systems (including self).
        for other in current_state.systems.iter() {
            todo!("check whether systems are interacting");
            // match system.variant {
            // SystemVariant::Planets(system) => match &other.variant {
            //     SystemVariant::Planets(other) => {}
            //     _ => todo!(),
            // },
            // _ => todo!(),
            // }
        }

        let variant = match &system.variant {
            // SystemVariant::Planets(_) => SystemVariant::Planets(self.forward_planets(system)),
            _ => todo!(),
        };
        System {
            system_id,
            variant,
            integrators,
        }
    }

    pub fn forward_planets(&self, system: &System) -> Planets {
        // match system.variant {
        // SystemVariant::Planets(_) => {}
        // _ => panic!(),
        // }

        let planets = Planets::new();
        let entities = planets
            .entities
            .iter()
            .map(|x| self.forward_planet(x))
            .collect();
        Planets { entities }
    }

    pub fn forward_planet(&self, planet: &Planet) -> Planet {
        let mass = todo!();
        let pos = todo!();
        let vel = todo!();
        let res = Planet::new(mass, pos, vel);
        res
    }

    pub fn send(&self) {}

    pub fn receive(&self) {}

    pub fn get_unsaved_state_ids(&self) -> Vec<usize> {
        let a = self
            .states
            .iter()
            .filter(|state| {
                state.state_id
                    >= match self.config.last_export_step_id {
                        None => 0,
                        Some(e) => e + 1,
                    }
            })
            .map(|state| state.state_id)
            .collect();
        a
    }
}
