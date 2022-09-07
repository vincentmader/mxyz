pub mod request;
pub mod response;
use serde::{Deserialize, Serialize};

/// TCP Package
#[derive(Debug, Serialize, Deserialize)]
pub enum TcpPackage {
    /// Request
    Request(request::Request),

    /// Response
    Response(response::Response),
}

impl TcpPackage {
    /// Convert TCP-Package to binary.
    pub fn to_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }

    /// Convert TCP-Package from binary.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        bincode::deserialize(&bytes[..]).unwrap()
    }
}
