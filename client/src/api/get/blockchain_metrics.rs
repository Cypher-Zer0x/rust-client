use crate::database::read_blocks::get_number_of_block;
use crate::database::read_transaction::get_number_tx;
use crate::database::read_utxo::get_number_UTXO;
use crate::interface::blockchain_metrics::blockchain_metrics::BlockchainMetrics;
use axum::response::Response;
use axum::{
    body::Body,
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, StatusCode},
    response::Json,
};

use serde_json;
use std::convert::Infallible;
// this function returns all the transactions in the mempool
pub fn get_blockchain_metrics() -> Result<serde_json::Value, lmdb::Error> {
    let numberOfBlock = get_number_of_block().unwrap();
    let numberOfTx = get_number_tx().unwrap();
    let numberOfUTXO = get_number_UTXO().unwrap();
    let data = BlockchainMetrics::new(numberOfBlock, numberOfTx, numberOfUTXO);
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_blockchain_metrics
pub async fn async_get_blockchain_metrics() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_blockchain_metrics).await {
        Ok(Ok(data)) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*") // Add this header for CORS handling
                .body(Body::from(Json(data).to_string())) // Use `.from(serde_json::to_string(&json).unwrap())` to set the response body as JSON
                .unwrap();

            // println!("response {:?}", response); // Consider handling this Result in production code
            Ok(response)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap()), // Consider handling this Result in production code
    }
}
