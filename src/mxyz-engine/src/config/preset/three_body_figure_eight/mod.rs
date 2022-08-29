use crate::config::EngineConfig;
use crate::system::System;

const NR_OF_STEPS: usize = 10;

pub fn preset(_systems: &mut Vec<System>, config: &mut EngineConfig) {
    config.step_id.1 = NR_OF_STEPS;
}
