use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SnapshotRow {
    pub id: u64,
    pub data_hash: String,
}

pub fn row_hash(row: &SnapshotRow) -> String {
    row.data_hash.clone()
}
