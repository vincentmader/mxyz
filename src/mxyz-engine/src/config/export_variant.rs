use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ExportVariant {
    ToDatabase,
    ToFile,
}
