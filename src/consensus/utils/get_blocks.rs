use web3::transports::batch;

use crate::{
    api::{get::block_data::get_block_by_number_data, requester::{get_block_by_index::get_block_by_index}},
    block_producer::block_producer::process_transaction,
    database::read::read_validators::get_validators, interface::Block,
};

// returns Vec<Block>
pub async fn get_blocks(
    validator_url: String,
    start_block_number: u128,
    max_block_number: u128,
    batch_size: u32,
) -> Result<Vec<Block>, reqwest::Error> {

    if start_block_number == max_block_number {
        return Ok(vec![get_block_by_index(validator_url, start_block_number as u128).await.unwrap()]);
    }
    // todo: handle batch_size = 0 and tother arg values

    // get the amount of api calls needed
    let mut api_calls: u128 = 0;
    // println!("max_block_number: {:?}", max_block_number);
    // println!("start_block_number: {:?}", start_block_number);
    
    if max_block_number - start_block_number > batch_size as u128
        && ((max_block_number - start_block_number) % batch_size as u128 != 0)
    {
        api_calls = (max_block_number - start_block_number) / batch_size as u128;
    } else if max_block_number - start_block_number > batch_size as u128 {
        api_calls = ((max_block_number - start_block_number) / batch_size as u128) + 1;
    } else {
        api_calls = 1;
    }

    // return new vec
    let mut blocks: Vec<Block> = Vec::new();

    for i in 0..api_calls {
        let start_block = start_block_number + (i * batch_size as u128);
        let mut end_block = start_block + batch_size as u128;
        let mut err_cpt = 0;

        if i == api_calls - 1 {
            end_block = max_block_number;
        }

        loop {
			// println!("ssssssssssssssssssssssssssss");
            // call api to get blocks
            let blocks_batch = crate::api::requester::get_block_range::get_block_range(
                validator_url.clone(),
                start_block as u32,
                end_block as u32,
            )
            .await;
			println!("blocks_batch: {:?}", blocks_batch);
            if blocks_batch.is_ok() {
                let blocks_batch = blocks_batch.unwrap();
                let blocks_batch = serde_json::from_str::<Vec<Block>>(&blocks_batch).unwrap();
                blocks.extend(blocks_batch);

                break;
            }

            // if the api call fails, retry 5 times
            err_cpt += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            if err_cpt == 5 {
                break; // todo: throw or try another validator
            }
        }
    }

    // println!("blocks: {:?}", blocks);

    return Ok(blocks);
}
