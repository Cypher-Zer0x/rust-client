use serde::{Deserialize, Serialize};

/// ExitClaimedEvent struct
/// This struct is used to represent the ExitClaimed event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExitClaimedEvent {
    pub owner: String,
    pub exit_id: String,
    pub amount: String,
}
