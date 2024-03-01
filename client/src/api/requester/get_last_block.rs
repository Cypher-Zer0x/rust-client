use reqwest;

use crate::interface::Block;

// query a node api to get the last block
pub async fn get_last_block(node_url: String) -> Result<Block, reqwest::Error> {
    let url = format!("http://{}/block/latest", node_url);
    // println!("get_last_block: {}", url);
    let res = reqwest::get(&url).await?;
    let body = res.text().await?;
    // println!("get_last_block: {:?}", body);
    let block = serde_json::from_str::<Block>(&body).unwrap();
    return Ok(block);
}
