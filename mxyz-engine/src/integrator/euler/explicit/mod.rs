use mxyz_universe::interaction::Interaction;
use mxyz_universe::state::State;
use mxyz_universe::system::system::System;

pub fn apply(
    sys_1: &System,
    current_state: &State,
    states: &Vec<State>,
    interactions: &Vec<Interaction>,
) -> System {
    for e_1 in &sys_1.entities {
        for sys_2 in current_state.systems.iter() {
            for e_2 in sys_2.entities.iter() {}
        }
    }

    System::new(sys_1.system_id, sys_1.variant.clone())
}
