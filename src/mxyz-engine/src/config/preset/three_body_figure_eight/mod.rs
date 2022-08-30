use crate::config::EngineConfig;
use crate::system::unsized_system::UnsizedSystem;

const NR_OF_STEPS: usize = 10;

pub fn preset(_systems: &mut Vec<UnsizedSystem>, config: &mut EngineConfig) {
    config.step_id.1 = NR_OF_STEPS;
}
