use bonsai_sdk::alpha as bonsai_sdk;
use bonsai_sdk::responses::SnarkReceipt;
use ethers::abi::Abi;
use ethers::core::types::U256;
use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use host::{run_bonsai, run_stark2snark};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::database::read_blocks;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Inputs {
    pub state_t_1: String, // merkle root of the state at t+1 (all the blocks)
    pub state_t: String,   // merkle root of the state at t (all the blocks)
    pub blocks_hash: Vec<String>, // hash of each block
}

impl Inputs {
    pub fn to_bytes(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}

pub async fn verify_on_chain(
    snark_receipt: SnarkReceipt,
) -> Result<(), Box<dyn std::error::Error>> {
    let contract_abi = r#"
    [{
		"inputs": [
			{
				"internalType": "uint256[2]",
				"name": "_pA",
				"type": "uint256[2]"
			},
			{
				"internalType": "uint256[2][2]",
				"name": "_pB",
				"type": "uint256[2][2]"
			},
			{
				"internalType": "uint256[2]",
				"name": "_pC",
				"type": "uint256[2]"
			},
			{
				"internalType": "uint256[4]",
				"name": "_pubSignals",
				"type": "uint256[4]"
			}
		],
		"name": "verifyProof",
		"outputs": [
			{
				"internalType": "bool",
				"name": "",
				"type": "bool"
			}
		],
		"stateMutability": "view",
		"type": "function"
	}
    ]
    "#;

    let provider = Provider::<Http>::try_from(
        "https://eth-sepolia.g.alchemy.com/v2/BS5hSVL2MXlbIl0VTnK4MCRqFSZLcMg-",
    )?;
    let provider = Arc::new(provider);
    let contract_address: H160 = "0x83c2e9cd64b2a16d3908e94c7654f3864212e2f8".parse::<Address>()?; //config.contract_address.parse::<Address>()?;
    let contract_abi: Abi = serde_json::from_str(contract_abi)?;
    let contract = Contract::new(contract_address, contract_abi, provider);

    /*let a0_bytes: [u8; 32] = snark_receipt.snark.a[0]
        .clone()
        .try_into()
        .expect("Incorrect length for a0");
    let a1_bytes: [u8; 32] = snark_receipt.snark.a[1]
        .clone()
        .try_into()
        .expect("Incorrect length for a1");
    let b0_bytes: [u8; 32] = snark_receipt.snark.b[0][0]
        .clone()
        .try_into()
        .expect("Incorrect length for b0");
    let b1_bytes: [u8; 32] = snark_receipt.snark.b[0][1]
        .clone()
        .try_into()
        .expect("Incorrect length for b1");
    let b2_bytes: [u8; 32] = snark_receipt.snark.b[1][0]
        .clone()
        .try_into()
        .expect("Incorrect length for b2");
    let b3_bytes: [u8; 32] = snark_receipt.snark.b[1][1]
        .clone()
        .try_into()
        .expect("Incorrect length for b3");
    let c0_bytes: [u8; 32] = snark_receipt.snark.c[0]
        .clone()
        .try_into()
        .expect("Incorrect length for c0");
    let c1_bytes: [u8; 32] = snark_receipt.snark.c[1]
        .clone()
        .try_into()
        .expect("Incorrect length for c1");
    let signal1_bytes: [u8; 32] = snark_receipt.snark.public[0]
        .clone()
        .try_into()
        .expect("Incorrect length for signal1");
    let signal2_bytes: [u8; 32] = snark_receipt.snark.public[1]
        .clone()
        .try_into()
        .expect("Incorrect length for signal2");
    let signal3_bytes: [u8; 32] = snark_receipt.snark.public[2]
        .clone()
        .try_into()
        .expect("Incorrect length for signal3");
    let signal4_bytes: [u8; 32] = snark_receipt.snark.public[3]
        .clone()
        .try_into()
        .expect("Incorrect length for signal4");*/

    /*let a0 = U256::from(a0_bytes);
    let a1 = U256::from(a1_bytes);
    let b0 = U256::from(b0_bytes);
    let b1 = U256::from(b1_bytes);
    let b2 = U256::from(b2_bytes);
    let b3 = U256::from(b3_bytes);
    let c0 = U256::from(c0_bytes);
    let c1 = U256::from(c1_bytes);
    let signal1 = U256::from(signal1_bytes);
    let signal2 = U256::from(signal2_bytes);
    let signal3 = U256::from(signal3_bytes);
    let signal4 = U256::from(signal4_bytes);*/
    let a0 = U256::from_dec_str(
        "9353649055001880451080369704879234343695026832373588152132355360449099814546",
    )
    .unwrap();
    let a1 = U256::from_dec_str(
        "6729718291741893553752757969774236388825948635082864656308585754431928423613",
    )
    .unwrap();
    let b0 = U256::from_dec_str(
        "10100653844049400855849050942094150368687454072879057546672969278423914793132",
    )
    .unwrap();
    let b1 = U256::from_dec_str(
        "13852281614479540232749139335805886622297829428827842772028759807664000000017",
    )
    .unwrap();
    let b2 = U256::from_dec_str(
        "14928493194307259659339531132063615201325939983689767244400030895764937334456",
    )
    .unwrap();
    let b3 = U256::from_dec_str(
        "20426008889232999290603480685613319778066435678866093591521561933134727893090",
    )
    .unwrap();
    let c0 = U256::from_dec_str(
        "13434088454671199877700923759645577138696837482808791274640989609436245133155",
    )
    .unwrap();
    let c1 = U256::from_dec_str(
        "16125521847031103002036015866222073309858247907722720156658871028125288432326",
    )
    .unwrap();
    let signal1 = U256::from_dec_str("91039097843120449453449593822342807849").unwrap();
    let signal2 = U256::from_dec_str("24946934259622365010039737625873252857").unwrap();
    let signal3 = U256::from_dec_str("303471869684994866908490898589058140899").unwrap();
    let signal4 = U256::from_dec_str("304204416563610904192848645937720932675").unwrap();
    // Call the contract's function
    match contract
        .method::<(
            [ethers::types::U256; 2],
            [[ethers::types::U256; 2]; 2],
            [ethers::types::U256; 2],
            [ethers::types::U256; 4],
        ), bool>(
            "verifyProof",
            (
                [a0, a1],
                [[b0, b1], [b2, b3]],
                [c0, c1],
                [signal1, signal2, signal3, signal4],
            ),
        )?
        .call()
        .await
    {
        Ok(result) => println!("Result: {:?}", result),
        Err(e) => println!("Failed to call contract method: {:?}", e),
    }
    Ok(())
}

pub async fn prove_state_diff(
    state_t_1: String,
    state_t: String,
    block_hash: Vec<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Assuming `Inputs` and `to_bytes()` are defined elsewhere and are non-blocking
    let input = Inputs {
        state_t_1: "0x123".to_string(),
        state_t: "0x456".to_string(),
        blocks_hash: vec!["0x789".to_string(), "0x101112".to_string()],
    };
    let input_data = input.to_bytes();

    // Use spawn_blocking for potentially blocking operations
    let session_uuid =tokio::task::spawn_blocking(move || {
        run_bonsai(input_data)
            .map_err(|e| e.to_string()) // Convert the error to String, which is Send + Sync
    }).await?; 
    let snark_data = tokio::task::spawn_blocking(move || {
        run_stark2snark(session_uuid?)
            .map_err(|e| e.to_string()) // Convert the error to String, which is Send + Sync
    }).await?; 

    // Assuming verify_on_chain is properly async
    verify_on_chain(snark_data?).await.expect("Failed to verify on chain");
    Ok(())
}

/*fn get_data_for_state_diff(){
    //first we get the last block
    let last_block = read_blocks::get_last_block();
    //we get the last block number when the state has been proven
    let last_block_proven = read_blocks::get_last_block_proven();
    //we get the last state proven
    let last_state_proven = read_blocks::get_last_state_proven();

    // then we get the merkle root of the state t+1
    let state_t_1 = read_blocks::get_state_merkle_root(last_block_proven);

    // then we get the hash of each block between t and t+1
    let blocks_hash = read_blocks::get_blocks_hash(last_block_proven, last_block);
    // then we call the prove_state_diff function
    prove_state_diff(state_t_1, last_state_proven, blocks_hash);
    // if the state is proven we update the state in the database
}*/
