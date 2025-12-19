use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SdnFlowId(pub [u8; 16]);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SdnEndpoint(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SdnPolicy {
    pub allowed: bool,
}
