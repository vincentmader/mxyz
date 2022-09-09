use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ExportVariant {
    ToDatabase,
    ToFile,
}
