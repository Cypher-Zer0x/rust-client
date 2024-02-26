use serde::{Deserialize, Serialize};

/// ValidatorAddedEvent struct
/// This struct is used to represent the ValidatorAdded event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorAddedEvent {
    pub owner: String,
    pub pubkey: String,
    pub staked_amount: String,
}
