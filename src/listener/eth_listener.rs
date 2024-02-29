use super::event_decoder::{
    decode_eth_deposit_created_event, decode_exit_claimed, decode_exit_request,
    decode_validator_added, decode_validator_exit_request,
};
use crate::database::write_mempool;
use crate::interface::event::EventSignature;
use web3::futures::StreamExt;
use web3::transports::ws::WebSocket;
use web3::{
    types::{Address, FilterBuilder, Log},
    Web3,
};

pub struct EthListener {
    web3: Web3<WebSocket>,
    contract_address: Address,
    chain_id: String,
}

impl EthListener {
    pub async fn new(node_url: &str, contract_address: &str, chain_id: String) -> Self {
        let transport = WebSocket::new(node_url).await.unwrap();
        let web3 = Web3::new(transport);
        let contract_address = contract_address.parse().unwrap();
        EthListener {
            web3,
            contract_address,
            chain_id,
        }
    }
    pub async fn listen_to_event(&self) -> web3::Result<()> {
        let filter = FilterBuilder::default()
            .address(vec![self.contract_address])
            .build();
        let sub = self.web3.eth_subscribe().subscribe_logs(filter).await?;
        println!("Listening for events...");

        sub.for_each_concurrent(None, |log| async move {
            match log {
                Ok(log) => {
                    self.handle_event(&log).await;
                }
                Err(e) => {
                    println!("Error: {:?}", e);
                }
            }
        })
        .await;

        Ok(())
    }
    async fn handle_event(&self, log: &Log) {
        // println!("Got log: {:?}", log);
        let event_signature_str = log.topics[0];
        // print!("Event signature: {:?}", event_signature_str);
        match event_signature_str {
            ref s if *s == EventSignature::DepositCreatedSignature.value() => {
                let event = decode_eth_deposit_created_event(&log, self.chain_id.clone());
                write_mempool::insert_user_deposit_mempool(event.unwrap())
                    .await
                    .unwrap();
                println!("Deposit tx in the mempool");
            }
            ref s if *s == EventSignature::ExitRequestEventSignature.value() => {
                // Extraction and handling for validator exit request event goes here.
                let event = decode_exit_request(&log);
                println!("Exit request event {:?}", event);
            }
            ref s if *s == EventSignature::ValidatorAddedEventSignature.value() => {
                let event = decode_validator_added(&log);
                println!("Validator added event {:?}", event);
            }
            ref s if *s == EventSignature::ValidatorExitRequestSignature.value() => {
                let event = decode_validator_exit_request(&log);
                println!("Validator exit request event {:?}", event);
            }

            // Add more match cases for other events if needed.
            _ => (),
        }
    }
}
