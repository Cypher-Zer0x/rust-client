use crate::interface::range_proof::LigthRangeProof;
use serde::{Deserialize, Serialize};
use web3::signing::keccak256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct PaymentUTXO {
    pub version: String,          // hex version number of the transaction
    pub transaction_hash: String, // hash of the transaction where this UTXO was output, coinbase transactions have a hash of 0
    pub output_index: u64,        // index number of the output in the transaction
    pub public_key: String, // (compressed point) -> a one-time public key generated for this transaction output
    pub unlock_time: Option<u64>,
    pub amount: String, // encrypted amount + blinding factor, only the owner can decrypt it (if coinbase, the amount is clear and there is no blinding factor)
    pub currency: String, // currency -> TODO: find a way to encrypt it too
    pub commitment: String, // (compressed point) -> a cryptographic commitment to the amount, allows verification without revealing the amount
    pub rangeProof: LigthRangeProof, // range proof of the amount
    pub rG: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct TempPaymentUTXO {
    pub version: String,          // hex version number of the transaction
    pub transaction_hash: String, // hash of the transaction where this UTXO was output, coinbase transactions have a hash of 0
    pub output_index: u64,        // index number of the output in the transaction
    pub public_key: String, // (compressed point) -> a one-time public key generated for this transaction output
    pub unlock_time: Option<u64>,
    pub amount: String, // encrypted amount + blinding factor, only the owner can decrypt it (if coinbase, the amount is clear and there is no blinding factor)
    pub currency: String, // currency -> TODO: find a way to encrypt it too
    pub commitment: String, // (compressed point) -> a cryptographic commitment to the amount, allows verification without revealing the amount
    pub rangeProof: LigthRangeProof, // range proof of the amount
    pub rG: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct ExitUTXO {
    pub transaction_hash: String, // hash of the transaction where this UTXO was output, coinbase transactions have a hash of 0
    pub output_index: u64,        // index number of the output in the transaction
    pub public_key: String, // (compressed point) -> a one-time public key generated for this transaction output
    pub unlock_time: Option<u64>,
    pub amount: String, // encrypted amount + blinding factor, only the owner can decrypt it (if coinbase, the amount is clear and there is no blinding factor)
    pub currency: String, // currency -> TODO: find a way to encrypt it too
    pub commitment: String, // (compressed point) -> a cryptographic commitment to the amount, allows verification without revealing the amount
    pub exitChain: String,  // the chain where the UTXO is exiting
    pub hash: String,       // hash of the UTXO
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct TempExitUTXO {
    pub transaction_hash: String, // hash of the transaction where this UTXO was output, coinbase transactions have a hash of 0
    pub output_index: u64,        // index number of the output in the transaction
    pub public_key: String, // (compressed point) -> a one-time public key generated for this transaction output
    pub unlock_time: Option<u64>,
    pub amount: String, // encrypted amount + blinding factor, only the owner can decrypt it (if coinbase, the amount is clear and there is no blinding factor)
    pub currency: String, // currency -> TODO: find a way to encrypt it too
    pub commitment: String, // (compressed point) -> a cryptographic commitment to the amount, allows verification without revealing the amount
    pub exitChain: String,  // the chain where the UTXO is exiting
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct CoinbaseUTXO {
    pub version: String,          // hex version number of the transaction
    pub transaction_hash: String, // hash of the transaction where this UTXO was output, coinbase transactions have a hash of 0
    pub output_index: u64,        // index number of the output in the transaction
    pub public_key: String, // (compressed point) -> a one-time public key generated for this transaction output
    pub unlock_time: Option<u64>,
    pub amount: String,     // coinbase amount is always clear
    pub currency: String, // todo: mask this too using the same method as the amount (xor concat(8bytesAmount, currencyId) and shared secret) -> How to prove the currency input = currency output ???
    pub commitment: String, // (compressed point) -> a cryptographic commitment to the amount, allows verification without revealing the amount
    pub rG: String,
    pub hash: String, // hash of the UTXO
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct TempCoinbaseUTXO {
    pub version: String,          // hex version number of the transaction
    pub transaction_hash: String, // hash of the transaction where this UTXO was output, coinbase transactions have a hash of 0
    pub output_index: u64,        // index number of the output in the transaction
    pub public_key: String, // (compressed point) -> a one-time public key generated for this transaction output
    pub amount: String,     // coinbase amount is always clear
    pub currency: String, // todo: mask this too using the same method as the amount (xor concat(8bytesAmount, currencyId) and shared secret) -> How to prove the currency input = currency output ???
    pub commitment: String, // (compressed point) -> a cryptographic commitment to the amount, allows verification without revealing the amount
    pub unlock_time: Option<u64>,
    pub rG: String,
}

impl PaymentUTXO {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<PaymentUTXO, Box<dyn std::error::Error>> {
        let utxo: PaymentUTXO = bincode::deserialize(bytes)?;
        Ok(utxo)
    }
    pub fn new(
        version: String,
        transaction_hash: String,
        output_index: u64,
        public_key: String,
        unlock_time: Option<u64>,
        amount: String,
        currency: String,
        commitment: String,
        rangeProof: LigthRangeProof,
        rG: String,
    ) -> Self {
        let temp_utxo = TempPaymentUTXO {
            version,
            transaction_hash,
            output_index,
            public_key,
            unlock_time,
            amount,
            currency,
            commitment,
            rangeProof,
            rG,
        };

        // Compute and set the hash
        let json_string = serde_json::to_string(&temp_utxo).unwrap();

        let hash = hex::encode(keccak256(json_string.as_bytes()));
        let utxo = PaymentUTXO {
            version: temp_utxo.version,
            transaction_hash: temp_utxo.transaction_hash,
            output_index: temp_utxo.output_index,
            public_key: temp_utxo.public_key,
            unlock_time: temp_utxo.unlock_time,
            amount: temp_utxo.amount,
            currency: temp_utxo.currency,
            commitment: temp_utxo.commitment,
            rangeProof: temp_utxo.rangeProof,
            rG: temp_utxo.rG,
            hash,
        };
        utxo
    }
}

impl ExitUTXO {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<ExitUTXO, Box<dyn std::error::Error>> {
        let utxo: ExitUTXO = bincode::deserialize(bytes)?;
        Ok(utxo)
    }

    pub fn new(
        transaction_hash: String,
        output_index: u64,
        public_key: String,
        unlock_time: Option<u64>,
        amount: String,
        currency: String,
        commitment: String,
        exitChain: String,
    ) -> Self {
        let mut temp_utxo = TempExitUTXO {
            transaction_hash,
            output_index,
            public_key,
            unlock_time,
            amount,
            currency,
            commitment,
            exitChain,
        };

        let json_string = serde_json::to_string(&temp_utxo).unwrap();
        let hash = hex::encode(keccak256(json_string.as_bytes()));
        let mut utxo = ExitUTXO {
            transaction_hash: temp_utxo.transaction_hash,
            output_index: temp_utxo.output_index,
            public_key: temp_utxo.public_key,
            unlock_time: temp_utxo.unlock_time,
            amount: temp_utxo.amount,
            currency: temp_utxo.currency,
            commitment: temp_utxo.commitment,
            exitChain: temp_utxo.exitChain,
            hash,
        };
        utxo
    }
}

impl CoinbaseUTXO {
    pub fn to_bytes(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let encoded = bincode::serialize(&self)?;
        Ok(encoded)
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<CoinbaseUTXO, Box<dyn std::error::Error>> {
        let utxo: CoinbaseUTXO = bincode::deserialize(bytes)?;
        Ok(utxo)
    }

    pub fn new(
        version: String,
        transaction_hash: String,
        output_index: u64,
        public_key: String,
        unlock_time: Option<u64>,
        amount: String,
        currency: String,
        commitment: String,
        rG: String,
    ) -> Self {
        let mut temp_utxo = TempCoinbaseUTXO {
            version,
            transaction_hash,
            output_index,
            public_key,
            unlock_time,
            amount,
            currency,
            commitment,
            rG,
        };

        let json_string = serde_json::to_string(&temp_utxo).unwrap();
        let hash = hex::encode(keccak256(json_string.as_bytes()));

        let mut utxo = CoinbaseUTXO {
            version: temp_utxo.version,
            transaction_hash: temp_utxo.transaction_hash,
            output_index: temp_utxo.output_index,
            public_key: temp_utxo.public_key,
            unlock_time: temp_utxo.unlock_time,
            amount: temp_utxo.amount,
            currency: temp_utxo.currency,
            commitment: temp_utxo.commitment,
            rG: temp_utxo.rG,
            hash,
        };
        utxo
    }
}

// Define an enum to encapsulate the different UTXO types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UTXO {
    Payment(PaymentUTXO),
    Exit(ExitUTXO),
    Coinbase(CoinbaseUTXO),
}

impl UTXO {
    // Serialization including a prefix
pub fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = Vec::new();
    match self {
        UTXO::Payment(_) => bytes.push(0),
        UTXO::Exit(_) => bytes.push(1),
        UTXO::Coinbase(_) => bytes.push(2),
    }
    bytes.extend(bincode::serialize(self).unwrap());
    bytes
}

// Deserialization using the prefix
pub fn from_bytes(bytes: &[u8]) -> Result<UTXO, Box<dyn std::error::Error>> {
    let (variant, data) = bytes.split_first().ok_or("Empty bytes array")?;
    println!("variant: {:?}", variant);
    let utxo = match variant {
        0 => bincode::deserialize::<UTXO>(data)?,
        1 => bincode::deserialize::<UTXO>(data)?,
        2 => bincode::deserialize::<UTXO>(data)?,
        _ => return Err("Unknown UTXO variant".into()),
    };
    Ok(utxo)
}

    
    pub fn hash(&self) -> Option<&str> {
        match self {
            UTXO::Coinbase(utxo) => Some(&utxo.hash),
            UTXO::Payment(utxo) => Some(&utxo.hash),
            UTXO::Exit(utxo) => Some(&utxo.hash),
            // Handle other variants as needed...
            _ => None, // Return None or handle as appropriate for other variants
        }
    }

    pub fn get_hash(&self) -> String {
        match self {
            UTXO::Coinbase(utxo) => utxo.hash.clone(),
            UTXO::Payment(utxo) => utxo.hash.clone(),
            UTXO::Exit(utxo) => utxo.hash.clone(),
            // Handle other variants as needed...
            _ => "".to_string(), // Return None or handle as appropriate for other variants
        }
    }
}
