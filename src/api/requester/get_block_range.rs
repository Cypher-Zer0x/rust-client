use reqwest;

// query a node api to get the last block
pub async fn get_block_range(
  node_url: String,
  min: u32,
  max: u32,
) -> Result<String, reqwest::Error> {
  let url = format!(
      "http://{}/block/range/{}",
      node_url,
      min.to_string() + "-" + &max.to_string()
  );
  let res = reqwest::get(url).await?.text().await?;
  return Ok(res);
}
