use crate::{
    api::requester::get_utxo_by_hash::get_utxo_by_hash, database::{insert_block, insert_user_deposit_tx, insert_utxo}, interface::{Block, Transaction, UserDepositTx, UTXO}
};
use web3::signing::keccak256;

pub async fn process_block(node_url: String, block: Block) -> Result<(), lmdb::Error> {
    // todo: check bloc validity

    // insert the block in the database
    let header_hash = hex::encode(keccak256(&block.header.to_bytes().unwrap()));
    let _ = insert_block(header_hash, block.header.block_number, block.clone());

    // insert each tx in the database
    for tx in block.transactions {
        // match tx {
        //     Transaction::UserDepositTx(user_deposit_tx) => {
        //         let _ = insert_user_deposit_tx(user_deposit_tx);
        //         let _ = insert_utxo(user_deposit_tx.output);
        //     }
        //     Transaction::RingCTx(ring) => {

        // add the tx to the transaction database
        match tx {
            Transaction::UserDeposit(user_deposit_tx) => {
                let _ = insert_user_deposit_tx(user_deposit_tx.clone());
                // get utxo from validator api
                let output = get_utxo_by_hash(node_url.clone(), user_deposit_tx.output).await.unwrap();
                let _ = insert_utxo(output);
            }
            Transaction::RingCT(ring) => {
                // todo
            }
        }
    }

    return Ok(());
}
