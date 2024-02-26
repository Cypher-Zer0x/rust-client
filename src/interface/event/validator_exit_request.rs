use serde::{Deserialize, Serialize};

/// ValidatorExitRequestEvent struct
/// This struct is used to represent the ValidatorExitRequest event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorExitRequestEvent {
    pub owner: String,
    pub amount: String,
    pub lock_time: String,
    pub pubkey: String,
}
