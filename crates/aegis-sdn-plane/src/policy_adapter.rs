use crate::model::{SdnFlowId, SdnPolicy};

pub fn allow_if_safe(_flow: &SdnFlowId, _policy: &SdnPolicy) -> bool { true }
