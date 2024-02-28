use crate::{
    api::requester::get_mempool::get_mempool, database::insert_transaction_in_mempool, interface,
};

pub async fn process_mempool(node_url: String) -> Result<(), lmdb::Error> {
    // get the mempool from contact node
    let mempool = get_mempool(node_url.clone()).await.unwrap();

    // save the mempool in the database
    for tx in mempool {
        let _ = insert_transaction_in_mempool(tx).unwrap();
    }

    return Ok(());
}
