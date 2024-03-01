mod api;
mod block_producer;
mod consensus;
mod database;
mod interface;
mod listener;
mod prover_poster;

use api::get::block_data::{
    async_get_block_by_hash_data, async_get_block_by_number_data, async_get_blocks_data,
    async_get_blocks_range_data, async_get_last_block_data, async_get_last_ten_blocks_data,
    async_get_number_block,
};
use api::get::blockchain_metrics::async_get_blockchain_metrics;
use api::get::mempool_data::async_get_mempool_data;
use api::get::transactions_data::{
    async_get_latest_ten_tx, async_get_number_tx, async_get_transaction_data,
    async_get_transaction_data_by_hash,
};
use api::get::state::{async_get_last_block_proven, async_get_last_state_proven};
use api::get::utxo_data::{async_get_utxo_data, async_get_utxo_data_by_hash};
use api::post::user_operation::handle_user_ringct;
use block_producer::block_producer::process_transaction;
use consensus::sync_with_network::sync_with_network;
use http::header::{ACCESS_CONTROL_ALLOW_HEADERS, CONTENT_TYPE};
use http::Method;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, Any, CorsLayer};
//use consensus::user_operationocks;
use api::requester::get_block_range::get_block_range;
use axum::{routing::get, routing::post, Router}; // Ensure this is correctly imported
use axum_server::Server;
use database::write::write_validator::insert_validator;
use dotenv::dotenv;
use interface::nodes::{Node, Validator};
use listener::eth_listener::EthListener;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Notify;
use tokio::time::Duration; // Add missing import statement
use prover_poster::run_prover;

#[derive(Serialize, Deserialize, Debug)]
struct NetworkConfig {
    node_url: String,
    contract_address: String,
    chain_id: String,
}

#[tokio::main(flavor = "multi_thread")] // Adjust the number of worker_threads as needed
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // todo: at start up, check database integrity (at least if all block indexes are here and utxo merkle root fit the one saved)
    dotenv().ok();
    let config_data = fs::read_to_string("network.json").expect("Unable to read network.json");
    let networks: Vec<NetworkConfig> =
        serde_json::from_str(&config_data).expect("Error parsing JSON");
    println!("{:?}", networks);
    // Setup database
    database::set_up_mldb()?;
    /*let node_url = env::var("WSS_PROVIDER").expect("WSS_PROVIDER not set");
    let contract_address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not set");*/
    let notify = Arc::new(Notify::new());

    // first we try to sync with the network
    let validator: Vec<Validator> = [Validator {
        node: Node {
            ip: "176.146.201.74".to_string(),
            port: "8000".to_string(),
            status: "".to_string(),
        },
        pubkey: "".to_string(),
        last_block_hash: "".to_string(),
        last_block_number: 0,
    }]
    .to_vec();
    let _ = insert_validator(validator);
    let _ = sync_with_network().await?;
    print!("synced with network");
    // let _ = get_block_range("127.0.0.1:8000".to_string(), 0, 10).await.unwrap();
    // let _ = get_blocks::get_blocks( "127.0.0.1:8000".to_string(),0 , 50, 10).await?;

    // Spawn the listener in a separate async task
    for config in networks {
        let node_url = config.node_url;
        let contract_address = config.contract_address;
        let chain_id = config.chain_id;

        tokio::spawn(async move {
            let listener = EthListener::new(&node_url, &contract_address, chain_id).await;
            listener
                .listen_to_event()
                .await
                .expect("Failed to listen to events");
        });
    }

    // Spawn the API server in a separate async task
    tokio::spawn(async {
        let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET,Method::POST,Method::OPTIONS]) // Specify allowed methods, including OPTIONS
        .allow_headers([CONTENT_TYPE,ACCESS_CONTROL_ALLOW_HEADERS]);  // Specify allowed headers
        let app = Router::new()
            /* ----------------------METRICS ENDPOINTS----------------- */
            .route("/metrics", get(async_get_blockchain_metrics))
            /* ----------------------TX ENDPOINTS---------------------- */
            .route("/mempool", get(async_get_mempool_data))
            .route("/utxo/set", get(async_get_utxo_data))
            .route("/utxo/hash/:hash", get(async_get_utxo_data_by_hash))
            .route("/transaction/all", get(async_get_transaction_data)) // todo: add range mechanism or at some point it will be too slow and inefficient for the client and the requester
            .route(
                "/transaction/hash/:tx_hash",
                get(async_get_transaction_data_by_hash),
            )
            .route("/transaction/number", get(async_get_number_tx))
            .route("/transaction/latest-ten", get(async_get_latest_ten_tx))
            /* ----------------------BLOCK ENDPOINTS---------------------- */
            .route("/block/all", get(async_get_blocks_data)) // todo: add range mechanism or at some point it will be too slow and inefficient for the client and the requester
            .route("/block/hash/:block_hash", get(async_get_block_by_hash_data))
            .route(
                "/block/number/:block_number",
                get(async_get_block_by_number_data),
            )
            .route("/block/range/:rangeData", get(async_get_blocks_range_data))
            .route("/block/latest", get(async_get_last_block_data))
            .route("/block/latest-ten", get(async_get_last_ten_blocks_data))
            .route("/block/total-number", get(async_get_number_block))
            /*-----------------------STATE ENDPOINTS----------------------- */
            .route("/state/current", get(async_get_last_state_proven))
            .route("/state/block/current", get(async_get_last_block_proven))
            /* ----------------------USER OPERATION ENDPOINTS---------------------- */
            .route("/ringct", post(handle_user_ringct))
            .layer(CorsLayer::permissive()); 
        let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
        println!("API server listening on {}", addr);
        // Run the server
        Server::bind(addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    });
    // Spawn the block producer in a separate async task
    tokio::spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
            let _ = process_transaction();
        }
    });
    // state diff prover/poster logic here, commented out for now
    tokio::spawn(async {
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let _ = run_prover().await;
        }
    });
    notify.notified().await;
    Ok(())
}
