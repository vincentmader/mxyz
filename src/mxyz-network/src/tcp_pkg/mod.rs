use mxyz_engine_universe::state::SizedState;
use serde::{Deserialize, Serialize};

pub mod request;
pub mod response;

#[derive(Debug, Serialize, Deserialize)]
pub enum TcpPackage {
    Request(request::Request),
    Response(response::Response),
    // Command(command::Command),
    // StateVec(Vec<SizedState>),
}
impl TcpPackage {
    pub fn to_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        bincode::deserialize(&bytes[..]).unwrap()
    }
}
