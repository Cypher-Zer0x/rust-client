use crate::database::read_transaction;
use axum::response::Response;
use axum::{
    body::Body,
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, StatusCode},
};
use axum::{extract::Path, response::Json};
use read_transaction::*;
use serde_json;
use std::convert::Infallible;
// this function returns all the transactions in the mempool
pub fn get_transaction_data() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_transactions().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_mempool_data
pub async fn async_get_transaction_data() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_transaction_data).await {
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
pub fn get_transaction_data_by_hash(tx_hash: String) -> Result<serde_json::Value, lmdb::Error> {
    let data = get_transaction_by_hash(tx_hash).unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_transaction_data_by_hash
pub async fn async_get_transaction_data_by_hash(
    Path(tx_hash): Path<String>,
) -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(move || get_transaction_data_by_hash(tx_hash)).await {
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

pub fn get_number_of_tx() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_number_tx().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

pub async fn async_get_number_tx() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_number_of_tx).await {
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

pub fn get_latest_ten_tx() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_latest_transactions().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

pub async fn async_get_latest_ten_tx() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_latest_ten_tx).await {
        Ok(Ok(data)) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*") // Add this header for CORS handling
                .body(Body::from(Json(data).to_string())) // Use `.from(serde_json::to_string(&json).unwrap())` to set the response body as JSON
                .unwrap();
            //println!("response {:?}", response); // Consider handling this Result in production code
            Ok(response)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap()), // Consider handling this Result in production code
    }
}
