use crate::database::read_utxo;
use axum::response::Response;
use axum::{
    body::Body,
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, StatusCode},
};
use axum::{extract::Path, response::Json};
use read_utxo::*;
use serde_json;
use std::convert::Infallible;
// this function returns all the transactions in the mempool
pub fn get_utxo_data() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_utxo_set().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_mempool_data
pub async fn async_get_utxo_data() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_utxo_data).await {
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

// this function returns the transaction with the given hash
pub fn get_utxo_data_by_hash(pubkey: String) -> Result<serde_json::Value, lmdb::Error> {
    let data = get_utxo_by_hash(pubkey).unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_transaction_data_by_hash
pub async fn async_get_utxo_data_by_hash(
    Path(pubkey): Path<String>,
) -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(move || get_utxo_data_by_hash(pubkey)).await {
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
