use mxyz_universe::system::System;

pub fn apply(system: &System) -> System {
    for entity in &system.entities {}

    System::new(system.system_id, system.variant.clone())
}
