// use mxyz_engine::state::preset::SimulationVariant;
pub mod command;

#[derive(Debug)]
pub enum MpscMessage {
    Command(command::Command),
    //     AddEngine(SimulationVariant),
    //     RemoveEngine(usize),

    //     TODO move stuff from tcp_package in here
}

// // pub type Message = Vec<u8>;
