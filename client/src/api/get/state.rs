use crate::database::read_state::{get_last_state_proven, get_last_block_proven};
use axum::response::Response;
use axum::{
    body::Body,
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, StatusCode},
    response::Json,
};
use serde_json;
use std::convert::Infallible;

// this function returns all the transactions in the mempool
pub fn get_last_state() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_last_state_proven().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_mempool_data
pub async fn async_get_last_state_proven() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_last_state).await {
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

// this function returns all the transactions in the mempool
pub fn get_last_block() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_last_block_proven().unwrap();
    let json = serde_json::to_value(data.to_string()).unwrap();
    Ok(json)
}

// async wrapper for get_mempool_data
pub async fn async_get_last_block_proven() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_last_block).await {
        Ok(Ok(data)) => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*") // Add this header for CORS handling
                .body(Body::from(Json(data).to_string())) // Use `.from(serde_json::to_string(&json).unwrap())` to set the response body as JSON
                .unwrap();
            Ok(response)
        }
        _ => Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap()), // Consider handling this Result in production code
    }
}