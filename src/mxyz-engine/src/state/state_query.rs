use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StateQuery {
    BatchSince(usize, usize),
    AllSince(usize),
    Between(usize, usize),
    // FromIds(Vec<i32>),
}
