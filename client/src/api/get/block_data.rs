use crate::database::read_blocks;
use axum::response::Response;
use axum::{
    body::Body,
    extract::Path,
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN, StatusCode},
    response::Json,
};
use read_blocks::*;
use serde_json;
use std::convert::Infallible;

pub fn get_last_block_data() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_last_block().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

pub async fn async_get_last_block_data() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_last_block_data).await {
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

// this function returns all the blocks in the blockchain
pub fn get_blocks_data() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_blocks().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_blocks_data
pub async fn async_get_blocks_data() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_blocks_data).await {
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

// this function returns the block with the given hash
pub fn get_block_by_hash_data(block_hash: String) -> Result<serde_json::Value, lmdb::Error> {
    let data = get_block_by_hash(block_hash).unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_block_by_hash_data
pub async fn async_get_block_by_hash_data(
    Path(block_hash): Path<String>,
) -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(move || get_block_by_hash_data(block_hash)).await {
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

// this function returns the block with the given number
pub fn get_block_by_number_data(block_number: u128) -> Result<serde_json::Value, lmdb::Error> {
    let data = get_block_by_number(block_number).unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// async wrapper for get_block_by_number_data
pub async fn async_get_block_by_number_data(
    Path(block_number): Path<u128>,
) -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(move || get_block_by_number_data(block_number)).await {
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

// get a range of blocks numbers
pub fn get_blocks_range(start: u128, end: u128) -> Result<serde_json::Value, lmdb::Error> {
    let mut actual_end = end;
    // check if the start and end index are in db
    let last_block = get_last_block_number().unwrap();
    if start > last_block.unwrap() {
        return Err(lmdb::Error::NotFound);
    }
    if end > last_block.unwrap() {
        actual_end = last_block.unwrap();
    }

    let mut data = Vec::new();
    for i in start..actual_end {
        let block = get_block_by_number(i).unwrap();
        data.push(block);
    }
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

// return a range of blocks
pub async fn async_get_blocks_range_data(
    Path(range_data): Path<String>,
) -> Result<Response, Infallible> {
    // range data is a string of the form "start-end"
    let range: Vec<&str> = range_data.split('-').collect();
    let start = range[0].parse::<u128>().unwrap();
    let end = range[1].parse::<u128>().unwrap();
    match tokio::task::spawn_blocking(move || get_blocks_range(start, end)).await {
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

pub fn get_last_ten_blocks_data() -> Result<serde_json::Value, lmdb::Error> {
    // Safely get the last block number
    let last_block_result = get_last_block_number();
    if let Err(e) = last_block_result {
        // If there's an error getting the last block number, return the error
        return Err(e);
    }
    let last_block = last_block_result.unwrap().unwrap();
    // Determine the starting block number, ensuring it doesn't attempt to fetch non-existing blocks
    let start_block = if last_block >= 11 { last_block - 10 } else { 1 };

    let mut data = Vec::new();
    for i in start_block..last_block + 1 {
        match get_block_by_number(i) {
            Ok(block) => data.push(block),
            Err(_) => {
                // Optionally log the error or handle it as necessary
                // Continue to the next iteration if a block cannot be fetched
                continue;
            }
        }
    }

    // Convert the collected data into JSON, handling potential serialization errors
    match serde_json::to_value(data) {
        Ok(json) => Ok(json),
        Err(_) => Err(lmdb::Error::Invalid),
    }
}

pub async fn async_get_last_ten_blocks_data() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_last_ten_blocks_data).await {
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

pub fn get_number_block() -> Result<serde_json::Value, lmdb::Error> {
    let data = get_number_of_block().unwrap();
    let json = serde_json::to_value(data).unwrap();
    Ok(json)
}

pub async fn async_get_number_block() -> Result<Response, Infallible> {
    match tokio::task::spawn_blocking(get_number_block).await {
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
