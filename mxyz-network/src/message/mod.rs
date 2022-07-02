use mxyz_engine::state::preset::SimulationVariant;

#[derive(Debug)]
pub enum Message {
    AddEngine(SimulationVariant),
    RemoveEngine(usize),
}
