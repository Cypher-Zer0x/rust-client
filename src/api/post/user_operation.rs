use crate::database::read_mempool::get_mempool;
use crate::database::read_utxo::get_utxo_by_hash;
use crate::database::write_mempool::insert_transaction_in_mempool;
use crate::interface::{PendingRingCT, PendingTransaction, VerifyTx, UTXO};
use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};
use axum::response::Response;
use axum::{
    body::Body,
    http::{header::ACCESS_CONTROL_ALLOW_ORIGIN},
};
use std::convert::Infallible;
pub async fn handle_user_ringct(
    payload: Json<PendingRingCT>,
) -> Result<Response,Infallible> {
    //println!("Received a ringCT transaction {:?}", payload);
    // println!("mempool before: {:?}", get_mempool().unwrap());

    let tx = PendingRingCT {
        inputs: payload.inputs.clone(),
        outputs: payload.outputs.clone(),
        hash: payload.hash.clone(),
        signature: payload.signature.clone(),
        fee: payload.fee.clone(),
    };
    // we check if the input exists in the database
    // and store the UTXO in a vector
    /* 
    if tx.inputs.len() == 0{
        return Ok(Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap());
    }
    if tx.outputs.len() == 0{
        return Ok(Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap());
    }
    if tx.signature == ""{
        return Ok(Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap());
    }
    */
    let mut inputs_utxo: Vec<String> = Vec::new();
    for input in tx.inputs.clone() {
        match get_utxo_by_hash(input) {
            Ok(utxo) => {
                // println!("UTXO found {:?}", utxo);
                inputs_utxo.push(utxo.get_commitment());
            }
            Err(e) => {
                // println!("UTXO not found {:?}", e);

            return Ok(Response::builder()
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap())
            }
        }
    }
    // we check if the signature is valid
    let verify_tx = VerifyTx {
        tx: payload.signature.to_string(), // Convert &str to String if necessary
        inputs: inputs_utxo,               // Clone if needed to satisfy ownership rules
        outputs: payload.outputs.clone(),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("http://127.0.0.1:3001/api/verifyTx")
        .json(&verify_tx)
        .send()
        .await;
    // println!("verify_tx after ts call: {:?}", res);

    // if the isValid field is false, we return an error
    if let Ok(res) = res {
        if res.status().is_success() {
            match res.json::<Value>().await {
                Ok(verify_response) => {
                    if verify_response["isValid"].as_bool().unwrap() {

                        // println!("Transaction is valid.");
                    } else {
                        // println!("Transaction is invalid.");

                        return Ok(Response::builder()
                        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap()); 
                    }
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON response: {}", err);
                    return Ok(Response::builder()
                    .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap());
                }
            }
        } else {
            eprintln!("Server returned an error: {:?}", res.status());
            return Ok(Response::builder()
            .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Internal Server Error"))
            .unwrap());
        }
    } else {
        // Handle network errors or other issues while sending the request.
        eprintln!("Error during the request");
        return Ok(Response::builder()
        .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::from("Internal Server Error"))
        .unwrap());
    }
    // if the signature is valid, we send the transaction to the mempool
    match insert_transaction_in_mempool(PendingTransaction::PendingRingCTx(tx.clone())) {
        Ok(_) => {
            let data = json!({
                "status": "success",
                "message": "Handled ringCT transaction"
            });
            // println!("mempool after: {:?}", get_mempool().unwrap());
            
            // Correctly construct the JSON response
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*") // Correct header name should be in quotes
                .body(Body::from(data.to_string())) // Serialize the `data` directly to a JSON string
                .unwrap()) // Assuming you want to unwrap here, but consider handling errors more gracefully
        },
        Err(e) => {
            eprintln!("Error inserting transaction in mempool: {:?}", e);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header(ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(Body::from(format!("{{\"error\":\"{}\"}}", e))) // Provide a JSON-formatted error message
                .unwrap()) // Again, consider a more graceful error handling
        }
    }

}
