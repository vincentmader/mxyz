use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum SimulationVariant {
    // Undefined, // TODO
    ThreeBodyFigureEight,
    ThreeBodyMoon,
}
// impl From<&str> for SimulationVariant {
//     fn from(a: &str) -> Self {
//         match a {
//             "3body-moon" => SimulationVariant::ThreeBodyMoon,
//             _ => Self::Undefined, // TODO
//         }
//     }
// }
