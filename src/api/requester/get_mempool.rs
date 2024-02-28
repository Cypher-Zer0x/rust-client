use crate::interface::pending_transactions::PendingTransaction;

// query a node api to get the last block
pub async fn get_mempool(node_url: String) -> Result<Vec<PendingTransaction>, reqwest::Error> {
    let url = format!("http://{}/mempool/", node_url);

    let res = reqwest::get(url).await?.text().await?;

    // Attempt to deserialize the response into a Vec<PendingTransaction>
    // Handle the case where the mempool is empty or the response cannot be parsed
    match serde_json::from_str::<Vec<PendingTransaction>>(&res) {
        Ok(mempool) => Ok(mempool),
        Err(e) => {
            // Check if the error is because of an empty response, which can be treated as an empty mempool
            if e.is_data() && e.to_string().contains("EOF while parsing a value") {
                // Return an empty vector to signify an empty mempool
                Ok(vec![])
            } else {
                // For other deserialization errors, panic or handle them as needed
                // Here, we choose to panic for simplicity, but you might want to handle this more gracefully
                //TODO HANDLE THE ERROR PROPERLY
                Ok(vec![])
            }
        }
    }
}
