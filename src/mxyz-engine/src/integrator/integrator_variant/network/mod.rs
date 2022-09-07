use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum NetworkIntegratorVariant {
    ConvolutionalNeuralNetwork,
    RecurrentNeuralNetwork,
}

impl ToString for NetworkIntegratorVariant {
    fn to_string(&self) -> String {
        match self {
            Self::ConvolutionalNeuralNetwork => "C.N.N.".into(),
            Self::RecurrentNeuralNetwork => "R.N.N.".into(),
        }
    }
}
