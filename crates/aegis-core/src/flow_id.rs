use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FlowId(pub [u8; 16]);

impl FlowId {
    pub fn new_zero() -> Self { FlowId([0u8;16]) }
}
