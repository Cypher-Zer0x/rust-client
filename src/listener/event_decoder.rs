use crate::interface::{
    ExitClaimedEvent, ExitRequestEvent, UserDepositEvent, ValidatorAddedEvent,
    ValidatorExitRequestEvent,
};
use keccak_hash::H256;
use std::vec;
use web3::ethabi::{self, Error, ParamType, Token};
use web3::types::Log;

fn h256_to_string(value: Option<H256>) -> String {
    match value {
        Some(hash) => format!("{:x}", hash), // Convert hash to hex string
        None => "None".to_string(),
    }
}

/*
    this function decodes the ETHDepositCreated event
    event DepositCreated(
        address owner, // msg.sender of the deposit
        uint amount, // amount of the deposit (in the smallest unit)
        string currency, // currency of the deposit
        uint blockNumber, // block number of the deposit
        string pubKey, // compressed public key of the owner of the utxo in the privacy layer
        string rG // random point used to create the commitment in the privacy layer
        );
*/
pub fn decode_eth_deposit_created_event(
    log: &Log,
    chain_id: String,
) -> Result<UserDepositEvent, Error> {
    // Define the parameter types of the ETHDepositCreated event
    let param_types = vec![
        ParamType::Address,   // owner
        ParamType::Uint(256), // amount
        ParamType::String,    // currency
        ParamType::Uint(256), // blockNumber
        ParamType::String,    // pubKey
        ParamType::String,    // rG
    ];

    // Decode the data part of the log
    let tokens = ethabi::decode(&param_types, &log.data.0)?;

    let amount = match &tokens[1] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let currency = match &tokens[2] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };
    let block_number = match &tokens[3] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let public_key = match &tokens[4] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };
    let r_g = match &tokens[5] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };

    let user_deposit_event = UserDepositEvent {
        txId: h256_to_string(log.transaction_hash),
        amount: amount.to_string(),
        currency,
        root_block_number: block_number.as_u64(),
        root_blockchain: chain_id,
        public_key,
        r_g,
    };
    Ok(user_deposit_event)
}

/*
    This function decodes the ExitRequest event
    event ExitRequest(
            address owner, // the address who can withdraw the funds from contract
            uint256 amount, // amount of the exit (in the smallest unit)
            string currency, // currency of the exit
            uint256 lockTime, // timestamp when the exit can be processed
            string pubKey // compressed public key of the owner of the utxo in the privacy layer
        );
*/

pub fn decode_exit_request(log: &Log) -> Result<ExitRequestEvent, Error> {
    let param_type = vec![
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::String,
        ParamType::Uint(256),
        ParamType::String,
    ];
    let tokens = ethabi::decode(&param_type, &log.data.0)?;

    // println!("tokens: {:?}", tokens);

    let owner = match &tokens[0] {
        Token::Address(addr) => *addr,
        _ => return Err(Error::InvalidData),
    };
    let amount = match &tokens[1] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let currency = match &tokens[2] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };
    let lock_time = match &tokens[3] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let public_key = match &tokens[4] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };

    let exit_request_event = ExitRequestEvent {
        owner: owner.to_string(),
        amount: amount.to_string(),
        currency,
        lock_time: lock_time.as_u64(),
        public_key,
    };
    Ok(exit_request_event)
}

/*
    This function decode the ValidatorAdded event
    event ValidatorAdded(
            address owner, // the address who can withdraw the funds from contract
            string pubKey, // compressed public key of the validator in the privacy layer
            uint256 stakedAmount // amount of the stake (in the smallest unit)
        );
*/
pub fn decode_validator_added(log: &Log) -> Result<ValidatorAddedEvent, Error> {
    let param_type = vec![ParamType::Address, ParamType::String, ParamType::Uint(256)];
    let tokens = ethabi::decode(&param_type, &log.data.0)?;

    // println!("tokens: {:?}", tokens);

    let owner = match &tokens[0] {
        Token::Address(addr) => *addr,
        _ => return Err(Error::InvalidData),
    };
    let pub_key = match &tokens[1] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };
    let staked_amount = match &tokens[2] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };

    let validator_added_event = ValidatorAddedEvent {
        owner: owner.to_string(),
        pubkey: pub_key,
        staked_amount: staked_amount.to_string(),
    };
    Ok(validator_added_event)
}

/*
    This function decode the ValidatorExitRequest event
    event ValidatorExitRequest(
            address owner, // the address who can withdraw the funds from contract
            uint256 amount, // amount of the exit (in the smallest unit)
            uint256 lockTime, // timestamp when the exit can be processed
            string pubKey // compressed public key of the validator in the privacy layer
        );
*/
pub fn decode_validator_exit_request(log: &Log) -> Result<ValidatorExitRequestEvent, Error> {
    let param_type = vec![
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Uint(256),
        ParamType::String,
    ];
    let tokens = ethabi::decode(&param_type, &log.data.0)?;

    // println!("tokens: {:?}", tokens);

    let owner = match &tokens[0] {
        Token::Address(addr) => *addr,
        _ => return Err(Error::InvalidData),
    };
    let amount = match &tokens[1] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let lock_time = match &tokens[2] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let pub_key = match &tokens[3] {
        Token::String(s) => s.clone(),
        _ => return Err(Error::InvalidData),
    };

    let validator_exit_request_event = ValidatorExitRequestEvent {
        owner: owner.to_string(),
        amount: amount.to_string(),
        lock_time: lock_time.to_string(),
        pubkey: pub_key,
    };
    Ok(validator_exit_request_event)
}

/*
    This function decode the ExitClaimed event
    event ExitClaimed(
            address owner, // the address who can withdraw the funds from contract
            uint256 exitId, // id of the exit
            uint256 amount // amount of the exit (in the smallest unit)
        );
*/
pub fn decode_exit_claimed(log: &Log) -> Result<ExitClaimedEvent, Error> {
    let param_type = vec![
        ParamType::Address,
        ParamType::Uint(256),
        ParamType::Uint(256),
    ];
    let tokens = ethabi::decode(&param_type, &log.data.0)?;

    // println!("tokens: {:?}", tokens);

    let owner = match &tokens[0] {
        Token::Address(addr) => *addr,
        _ => return Err(Error::InvalidData),
    };
    let exit_id = match &tokens[1] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };
    let amount = match &tokens[2] {
        Token::Uint(u) => *u,
        _ => return Err(Error::InvalidData),
    };

    let exit_claimed_event = ExitClaimedEvent {
        owner: owner.to_string(),
        exit_id: exit_id.to_string(),
        amount: amount.to_string(),
    };
    Ok(exit_claimed_event)
}
