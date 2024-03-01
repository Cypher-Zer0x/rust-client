use serde::{Deserialize, Serialize};

/// ExitRequestEvent struct
/// This struct is used to represent the ExitRequest event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitRequestEvent {
    pub owner: String,
    pub amount: String,
    pub currency: String,
    pub lock_time: u64,
    pub public_key: String,
}
