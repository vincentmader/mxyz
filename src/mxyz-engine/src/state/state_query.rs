use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum StateQuery {
    BatchSince(i32, i32),
    AllSince(i32),
    Between(i32, i32),
    FromIds(Vec<i32>),
}
