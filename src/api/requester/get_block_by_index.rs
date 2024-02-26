use reqwest;

use crate::interface::Block;

// query a node api to get the last block
pub async fn get_block_by_index(node_url: String, block_index: u128) -> Result<Block, reqwest::Error> {
  let url = format!("http://{}/block/number/{}", node_url, block_index.to_string());
  // println!("get_block_range: {}", url);
  let res = reqwest::get(url).await?.text().await?;
  println!("get_block_range: {:?}", res);
  // println!("get_block_range: {:?}", res);
  return Ok(serde_json::from_str(&res).unwrap());
}