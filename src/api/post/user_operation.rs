use crate::database::read_utxo::get_utxo_by_hash;
use crate::database::write_mempool::insert_transaction_in_mempool;
use crate::interface::{PendingRingCT, PendingTransaction, VerifyTx, UTXO};
use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

pub async fn handle_user_ringct(
    payload: Json<PendingRingCT>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    //println!("Received a ringCT transaction {:?}", payload);

    let tx = PendingRingCT {
        inputs: payload.inputs.clone(),
        outputs: payload.outputs.clone(),
        hash: payload.hash.clone(),
        signature: payload.signature.clone(),
        fee: payload.fee.clone(),
    };
    // we check if the input exists in the database
    // and store the UTXO in a vector
    let mut inputs_utxo: Vec<String> = Vec::new();
    for input in tx.inputs.clone() {
        match get_utxo_by_hash(input) {
            Ok(utxo) => {
                // println!("UTXO found {:?}", utxo);
                inputs_utxo.push(utxo.get_commitment());
            }
            Err(e) => {
                // println!("UTXO not found {:?}", e);

                return Err(StatusCode::BAD_REQUEST);
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
    // if the isValid field is false, we return an error
    if let Ok(res) = res {
        if res.status().is_success() {
            match res.json::<Value>().await {
                Ok(verify_response) => {
                    if verify_response["isValid"].as_bool().unwrap() {

                        // println!("Transaction is valid.");
                    } else {
                        // println!("Transaction is invalid.");

                        return Err(StatusCode::BAD_REQUEST);
                    }
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON response: {}", err);
                    return Err(StatusCode::BAD_REQUEST);
                }
            }
        } else {
            eprintln!("Server returned an error: {:?}", res.status());
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        // Handle network errors or other issues while sending the request.
        eprintln!("Error during the request");
        return Err(StatusCode::BAD_REQUEST);
    }

    // if the signature is valid, we send the transaction to the mempool
    match insert_transaction_in_mempool(PendingTransaction::PendingRingCTx(tx.clone())) {
        Ok(_) => {
            return Ok(Json(json!(
                {
                    "status": "success",
                    "message": "Handled ringCT transaction"
                }
            )));
        }
        Err(e) => {
            eprintln!("Error inserting transaction in mempool: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}
