use crate::consensus::utils::process_block::process_block;
use crate::consensus::utils::process_mempool::process_mempool;
use crate::database::read_validators::get_validators;
use crate::database::{self, delete_blocks};
use crate::interface::nodes::{Node, Validator};
use crate::interface::Block;
use crate::api::requester::get_last_block::get_last_block;
use crate::{
    block_producer::block_producer::process_transaction,
};

use crate::consensus::utils::get_blocks::get_blocks;

/*
Stuff to verify while syncing the blockchain:
- each tx validity
- each block validity
- check all the events from the blockchain to build a local state of validator list to check if each block creator is the right one (the first in the validator list)

For now: blindly trust the network: get validator list from another validator, get blocks and add them to the local blockchain.
Once the blockchain is synced, emit deposit on mainnet to be added to the validator list.
 */

// this function sync the validator with the network
pub async fn sync_with_network() -> Result<(), lmdb::Error> {
    // get the local last block hash and number

    // ask a validator for the last block hash and number
    let validators: Vec<Validator> = [Validator {
        node: Node {
            ip: "176.146.201.74".to_string(),
            port: "3000".to_string(),
            status: "".to_string(),
        },
        pubkey: "".to_string(),
        last_block_hash: "".to_string(),
        last_block_number: 0,
    }]
    .to_vec(); // get_validators().unwrap();
    // println!("validators: {:?}", validators);

    if validators.len() == 0 {
        println!("No validators found. Starting the blockchain from genesis");
        return Ok(());
    }
    // get the last block hash and last block number from the local blockchain
    let mut local_last_block_number: u128 = database::read::read_blocks::get_last_block_number()
        .unwrap()
        .unwrap();
    // let local_last_block_hash: String = database::read::read_blocks::get_last_block_hash()
    //     .unwrap()
    //     .unwrap();

    // try to get the last block hash and last block number from the min(10, validators.length) first validator (oly save the highest block number).
    // if the requests fail, try the next validator. If all validators fail, start the blockchain from genesis
    let mut best_block_number: u128 = 0;
    // let mut last_block_hash = String::new();
    let mut best_validator = validators[0].clone().node.ip + ":" + &validators[0].clone().node.port;
    for index in 0..std::cmp::min(10, validators.len()) {
        let validator = validators[index].clone();
        let last_block =
            get_last_block(validator.node.ip.clone() + ":" + &validator.node.port).await;
        println!("validator last block: {:?}", last_block);
        if last_block.is_ok() {
            let last_block_unwrapped = &last_block.unwrap();
            if last_block_unwrapped.header.block_number > best_block_number {
                best_block_number = last_block_unwrapped.header.block_number;
                best_validator = validator.node.ip + ":" + &validator.node.port;
            }
        }
    }

    // if the best block number is 0, start the blockchain from genesis
    if best_block_number == 0 {
        println!("No valid block found. Starting the blockchain from genesis ..");
        return Ok(());
    }
    local_last_block_number = database::read::read_blocks::get_last_block_number()
        .unwrap()
        .unwrap();

    // if the local last block is > than the last valid block, delete all the blocks from the local blockchain
    // and sync from genesis
    if local_last_block_number > best_block_number {
        println!(
            "Local blockchain is ahead of the network. Starting the blockchain from genesis .."
        );
        let mut indexes: Vec<String> = vec![];
        for i in local_last_block_number..best_block_number + 1 {
            indexes.push(i.to_string());
        }
        let _ = delete_blocks(indexes);
    }

    // ask for batches of 100 blocks
    let mut tries = 0;
    println!("local_last_block_number before loop: {:?}", local_last_block_number);
    loop {
        // sync the blockchain
        let blocks = get_blocks(
            best_validator.clone(),
            local_last_block_number + 1,
            best_block_number,
            100,
        )
        .await;
        println!("blocks ok?: {:?}", blocks.is_ok());
        if blocks.is_ok() {
            // todo
            for block in blocks.unwrap() {
                // todo: verify block validity


                // check if the block number is the next one in the local blockchain
                local_last_block_number += 1;
                if local_last_block_number != block.header.block_number {
                    println!("Block number mismatch. Expected: {}, got: {}", local_last_block_number, block.header.block_number);
                    break;
                }

                println!("block: {:?}", block.header.block_number);

                // process the block
                let _ = process_block(best_validator.clone(), block);

                println!("local_last_block_number: {:?}", local_last_block_number);

                if best_block_number as i32 - local_last_block_number as i32 == 0 {
                    break;
                }
            }

            // copy mempool
            let _ = process_mempool(best_validator.clone()).await;

            // if the block is invalid: get the last blocks from each known validator except the last validator I used and keep syncing from the last valid block

            tries = 0;
            // println!("blocks: {:?}", blocks);
        } else {
            tries += 1;
            println!("Failed to get blocks from the network. retrying in 1 second");
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        }

        if tries > 10 { // todo
             // The validator is not responding. Get the last blocks from each known validator except the last validator I used and keep syncing from the last valid block

            // if all validators fail, start the blockchain from last valid block

            // if the local last block is > than the last valid block, start the blockchain from the local last block
        }

        if best_block_number as i32 - local_last_block_number as i32 == 0 {
            break;
        }
    }

    // println!("blocks: {:?}", blocks);
    return Ok(());
}
