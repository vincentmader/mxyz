#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};

/// Physical Field
///
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum PhysicalField {
    ElectroMagnetism(Vec<SimulationVariant>),
    QuantumMechanics(Vec<SimulationVariant>),
    NewtonianGravity(Vec<SimulationVariant>),
    Oscillators(Vec<SimulationVariant>),
    ThermoDynamics(Vec<SimulationVariant>),
    FluidDynamics(Vec<SimulationVariant>),
    EmergentBehavior(Vec<SimulationVariant>),
    MachineLearning(Vec<SimulationVariant>),
}
impl PhysicalField {
    pub fn get_all() -> Vec<PhysicalField> {
        vec![
            Self::NewtonianGravity(vec![]),
            Self::Oscillators(vec![]),
            Self::ElectroMagnetism(vec![]),
            Self::FluidDynamics(vec![]),
            Self::ThermoDynamics(vec![]),
            Self::QuantumMechanics(vec![]),
            Self::EmergentBehavior(vec![]),
            Self::MachineLearning(vec![]),
            // NOTE keep updated with above enum definition
        ]
    }
}
impl PhysicalField {
    pub fn to_string(&self) -> String {
        match self {
            Self::EmergentBehavior(_) => "emergent behavior",
            Self::FluidDynamics(_) => "fluid dynamics",
            Self::ElectroMagnetism(_) => "electro-magnetism",
            Self::QuantumMechanics(_) => "quantum mechanics",
            Self::NewtonianGravity(_) => "Newtonian gravity",
            Self::Oscillators(_) => "oscillators",
            Self::ThermoDynamics(_) => "thermo-dynamics",
            Self::MachineLearning(_) => "machine learning",
        }
        .to_owned()
    }
}

/// Simulation Variant
/// - basically an ID for each different simulation setup/preset
///
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SimulationVariant {
    ThreeBodyFigureEight,
    ThreeBodyMoon,
    SymmetricSatellites,
    ChargeInteraction,
    IsingModel,
    Boids,
    Ants,
    BrownianMotion,
    ThermalMotion,
    QuantumMechanicalHarmonicOscillator,
    HydrogenEnergyLevels,
    SinglePendulum,
    DoublePendulum,
    HarmonicOscillator,
    FourierTransform,
    LissajousFigures,
    SolarSystem,
    WienFilter,
    Magnet,
    GameOfLife,
    RockPaperScissors,
    Diffusion,
    AsteroidsAroundBinary,
    ChessBot,
    DrawnDigitClassifier,
    DrawnObjectClassifier,
    MonteCarloPiCalculation,
}
impl SimulationVariant {
    pub fn get_by_physical_field(field: PhysicalField) -> Vec<SimulationVariant> {
        match field {
            PhysicalField::ElectroMagnetism(_) => vec![
                Self::ChargeInteraction,
                Self::IsingModel,
                Self::WienFilter,
                Self::Magnet,
            ],
            PhysicalField::QuantumMechanics(_) => vec![
                Self::QuantumMechanicalHarmonicOscillator,
                Self::HydrogenEnergyLevels,
            ],
            PhysicalField::Oscillators(_) => vec![
                Self::DoublePendulum,
                Self::SinglePendulum,
                Self::HarmonicOscillator,
                Self::FourierTransform,
                Self::LissajousFigures,
            ],
            PhysicalField::NewtonianGravity(_) => vec![
                Self::ThreeBodyMoon,
                Self::ThreeBodyFigureEight,
                Self::SymmetricSatellites,
                Self::SolarSystem,
                Self::AsteroidsAroundBinary,
            ],
            PhysicalField::ThermoDynamics(_) => vec![Self::BrownianMotion, Self::ThermalMotion],
            PhysicalField::FluidDynamics(_) => vec![Self::Diffusion],
            PhysicalField::EmergentBehavior(_) => vec![
                Self::Boids,
                Self::Ants,
                Self::GameOfLife,
                Self::RockPaperScissors,
            ],
            PhysicalField::MachineLearning(_) => vec![
                Self::ChessBot,
                Self::DrawnDigitClassifier,
                Self::DrawnObjectClassifier,
            ],
        }
    }
}
impl SimulationVariant {
    pub fn into_short_description_string(&self) -> String {
        match self {
            Self::ThreeBodyFigureEight => "figure-8 orbit",
            Self::ThreeBodyMoon => "sun-moon-earth",
            Self::SymmetricSatellites => "symmetric sattelites",
            Self::ChargeInteraction => "charge interaction",
            Self::IsingModel => "Ising model",
            Self::Boids => "boids",
            Self::Ants => "ants",
            Self::BrownianMotion => "Brownian motion",
            Self::ThermalMotion => "thermal motion",
            Self::HydrogenEnergyLevels => "hydrogen energy levels",
            Self::QuantumMechanicalHarmonicOscillator => "QM harmonic oscillator",
            Self::HarmonicOscillator => "harmonic oscillator",
            Self::DoublePendulum => "double pendulum",
            Self::SinglePendulum => "single pendulum",
            Self::FourierTransform => "Fourier transform",
            Self::LissajousFigures => "Lissajous figures",
            Self::SolarSystem => "solar system",
            Self::WienFilter => "Wien filter",
            Self::Magnet => "magnet",
            Self::GameOfLife => "game of life",
            Self::RockPaperScissors => "rock-paper-scissors",
            Self::Diffusion => "diffusion",
            Self::AsteroidsAroundBinary => "asteroids around binary",
            Self::DrawnObjectClassifier => "drawn-object classifier",
            Self::DrawnDigitClassifier => "drawn-digit classifier",
            Self::ChessBot => "chess bot",
            Self::MonteCarloPiCalculation => "Monte Carlo pi",
        }
        .to_owned()
    }
    pub fn into_thumbnail_filename(&self) -> String {
        let filename = match self {
            Self::ChargeInteraction => "nbody-charge-interaction",
            Self::IsingModel => "ising-model",
            Self::WienFilter => "wien-filter",
            Self::Diffusion => "diffusion",
            Self::DoublePendulum => "double-pendulum",
            Self::LissajousFigures => "lissajous-figures",
            Self::ThermalMotion => "thermal-motion",
            Self::BrownianMotion => "brownian-motion",
            Self::RockPaperScissors => "rock-paper-scissors",
            Self::SolarSystem => "solar-system",
            Self::SinglePendulum => "single-pendulum",
            Self::GameOfLife => "game-of-life",
            Self::ThreeBodyMoon => "3body-moon",
            Self::ThreeBodyFigureEight => "3body-fig8",
            Self::AsteroidsAroundBinary => "nbody-binary",
            Self::SymmetricSatellites => "nbody-satellites",
            Self::MonteCarloPiCalculation => "pi-calculation",
            Self::Boids => "boids",
            Self::Ants => "ants",
            _ => "default",
        };
        let path = format!("img/simulations/{}/{}.png", filename, filename);
        path
    }
}
impl std::convert::Into<usize> for SimulationVariant {
    // NOTE: This is only used for to-file engine exports (directory names) at the moment.
    fn into(self) -> usize {
        match self {
            Self::ThreeBodyMoon => 0,
            Self::ThreeBodyFigureEight => 1,
            Self::ChargeInteraction => 2,
            Self::IsingModel => 3,
            Self::Boids => 4,
            _ => todo!(),
        }
    }
}
impl From<usize> for SimulationVariant {
    // NOTE: This is not used at the moment.
    fn from(simulation_variant: usize) -> Self {
        match simulation_variant {
            0 => Self::ThreeBodyMoon,
            1 => Self::ThreeBodyFigureEight,
            2 => Self::ChargeInteraction,
            3 => Self::IsingModel,
            4 => Self::Boids,
            _ => todo!(),
        }
    }
}
impl From<&str> for SimulationVariant {
    fn from(simulation_variant: &str) -> Self {
        match simulation_variant {
            "3body-moon" => Self::ThreeBodyMoon,
            "3body-fig8" => Self::ThreeBodyFigureEight,
            "nbody-charge-interaction" => Self::ChargeInteraction,
            "ising-model" => Self::IsingModel,
            "symmetric-satellites" => Self::SymmetricSatellites,
            "nbody-boids" => Self::Boids,
            _ => todo!(),
        }
    }
}
