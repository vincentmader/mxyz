pub mod request;
pub mod response;
use serde::{Deserialize, Serialize};
// -----------------------------------------------------------------------------
#[derive(Debug, Serialize, Deserialize)]
pub enum TcpPackage {
    Request(request::Request),
    Response(response::Response),
}
impl TcpPackage {
    pub fn to_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        bincode::deserialize(&bytes[..]).unwrap()
    }
}
