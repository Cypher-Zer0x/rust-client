use crate::database::insert_ring_ct;
use crate::interface;
use crate::interface::{
    block::Block, block::BlockHeader, finalized_transaction::Transaction, ringCTx,
};
use crate::{
    database::{
        read_blocks::{get_last_block_hash, get_last_block_number},
        read_mempool::get_mempool,
        write::{
            write_block::insert_block, write_mempool::delete_transaction_from_mempool,
            write_transaction::insert_user_deposit_tx, write_utxo::insert_utxo,
        },
    },
    interface::{PendingTransaction, UserDepositTx},
};
use eth_merkle_tree::tree::MerkleTree;
use std::time::SystemTime;
use web3::signing::keccak256;

// this function processes the transactions in the mempool and creates a block
pub fn process_transaction() -> Result<(), lmdb::Error> {
    let pending_txs = get_mempool().unwrap();
    // println!("pending_txs: {:?}", pending_txs);
    let mut finalized_txs: Vec<Transaction> = vec![];
    let mut hashes: Vec<String> = vec![];
    for tx in pending_txs {
        match tx {
            // for user deposit tx we need to create the output and add it to the utxo set
            // so only for this kind of tx we need to have the output and not the hash of the output
            PendingTransaction::PendingDeposit(deposit_tx) => {
                //here this is a user deposit, no need to check the validity, for the moment has it comes from the client
                // first add the output to the utxo set
                let _ = insert_utxo(deposit_tx.clone().output);
                // then add the tx to the transaction database, and to the vector of finalized txs
                let finalized_transaction =
                    UserDepositTx::from_pending_user_deposit_tx(deposit_tx.clone());
                let _ = insert_user_deposit_tx(finalized_transaction.clone());
                finalized_txs.push(Transaction::from_user_deposit_tx(finalized_transaction));
                //then delete the tx from the mempool
                let _ = delete_transaction_from_mempool(deposit_tx.clone().hash);
                // add the hash to the hashes vector
                hashes.push(deposit_tx.hash);
            }
            PendingTransaction::PendingRingCTx(ringCT) => {
                // we check if the input are in the database,
                // if so we send a request to the node service to verify the validity of the signature and of the CT
                // insert all utxos in the database
                for output in ringCT.outputs.clone() {
                    let _ = insert_utxo(interface::utxo::utxo::UTXO::Payment(output.clone()));
                }
                // then add the tx to the transaction database, and to the vector of finalized txs
                let finalized_transaction: ringCTx = ringCTx::from_pending_ringCTx(ringCT.clone());
                let _ = insert_ring_ct(finalized_transaction.clone());

                finalized_txs.push(Transaction::from_ringCTx(finalized_transaction));
                //then delete the tx from the mempool
                let _ = delete_transaction_from_mempool(ringCT.clone().hash);

                // add the hash to the hashes vector
                hashes.push(ringCT.hash);
            }
        }
    }
    // we have all the hashes of the transactions that we processed
    // we can now create the block
    // we need to get the last block hash and the last block number
    // Handling the last block hash with Option type
    let last_block_hash = match get_last_block_hash() {
        Ok(Some(hash)) => hash,
        Ok(None) => {
            println!("No previous block found. This might be the first block.");
            "GENESIS".to_string() // Using a string "0" to indicate no previous block
        }
        Err(e) => {
            eprintln!("Error retrieving last block hash: {:?}", e);
            return Err(e); // Propagating the error upwards
        }
    };
    let block_number = match get_last_block_number() {
        Ok(Some(number)) => number + 1,
        Ok(None) => {
            println!("No previous block found. This might be the first block.");
            0 // Using a string "0" to indicate no previous block
        }
        Err(e) => {
            eprintln!("Error retrieving last block number: {:?}", e);
            return Err(e); // Propagating the error upwards
        }
    };
    // we need to get the timestamp
    let timestamp = get_timestamp();
    // we need to create the merkle root
    let root = get_merkle_root(hashes);
    // we create the block header
    let header = BlockHeader {
        parent_block: last_block_hash,
        block_number,
        timestamp,
        merkle_root: root,
    };
    let header_hash = hex::encode(keccak256(&header.to_bytes().unwrap()));
    let block = Block {
        header,
        transactions: finalized_txs.to_vec(),
        hash: header_hash.clone(),
    };
    let _ = insert_block(header_hash, block_number, block);
    Ok(())
}

// this function returns the current timestamp
pub fn get_timestamp() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}

// this function returns the merkle root of a list of hashes
pub fn get_merkle_root(hashes: Vec<String>) -> String {
    if hashes.is_empty() {
        return "".to_string();
    }
    let tree = MerkleTree::new(&hashes).expect("Tree creation error.");
    let root = tree.root.expect("Unable to access root");
    root.data
}
