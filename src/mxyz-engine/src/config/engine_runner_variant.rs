#[derive(Default, Clone)]
pub enum EngineRunnerVariant {
    #[default]
    ClientWASM,
    ServerWASM,
    ServerRust,
    LocalRust,
    LocalWASM,
}
