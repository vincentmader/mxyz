use serde::{Deserialize, Serialize};
pub mod request;
pub mod response;

#[derive(Serialize, Deserialize)]
pub enum Package {
    Request(request::Request),
    Response(response::Response),
}
impl Package {
    pub fn to_bytes(self) -> Vec<u8> {
        bincode::serialize(&self).unwrap()
    }
    pub fn from_bytes(self, bytes: Vec<u8>) -> Self {
        bincode::deserialize(&bytes[..]).unwrap()
    }
}