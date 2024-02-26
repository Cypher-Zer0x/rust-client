use keccak_hash::H256;
use std::str::FromStr;
///
/// EventSignature enum contains the event signatures of the contract events
///
pub enum EventSignature {
    DepositCreatedSignature,
    ValidatorExitRequestSignature,
    ValidatorAddedEventSignature,
    ExitRequestEventSignature,
}

///
/// EventSignature implementation
///
impl EventSignature {
    pub fn value(&self) -> H256 {
        match self {
            EventSignature::DepositCreatedSignature => {
                H256::from_str("0x78111884e0b822e189661b668857f336e5efb56cb308331769b0432b6fd58f8e")
                    .unwrap()
            }
            EventSignature::ValidatorExitRequestSignature => {
                H256::from_str("0xb0355cf60ec2e58bffc9ef225bbb2e54af08fa38c631aac9a565216c377cbfcc")
                    .unwrap()
            }
            EventSignature::ValidatorAddedEventSignature => {
                H256::from_str("0x35300c28706ba3481d195ff2c3efb96b620acb9130db6377237936ba3cdd85bb")
                    .unwrap()
            }
            EventSignature::ExitRequestEventSignature => {
                H256::from_str("0x14d56b055f1f36da759d5bd7a99af08c11d206774b8202acb670640dc0206b5f")
                    .unwrap()
            }
        }
    }
}
