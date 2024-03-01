use reqwest;

use crate::interface::UTXO;

// query a node api to get the last block
pub async fn get_utxo_by_hash(node_url: String, utxo_hash: String) -> Result<UTXO, reqwest::Error> {
    let url = format!("http://{}/utxo/hash/{}", node_url, utxo_hash);
    let res = reqwest::get(url).await?.text().await?;

    let utxo: UTXO = serde_json::from_str(&res).unwrap();
    return Ok(utxo);
}
