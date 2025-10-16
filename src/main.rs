use std::str::FromStr;

// use cargo_contract::*;
use contract_extrinsics::{CallCommandBuilder, CallExec, ExtrinsicOptsBuilder};
use hex::FromHex;
use ink_env::{DefaultEnvironment, Environment};
use subxt::{Config, PolkadotConfig as DefaultConfig, tx, utils::H160};
use subxt_signer::{SecretUri, sr25519::Keypair};

use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let contract_address_str = "64EaB2421807bEFA142970f5e4ff3c8C18Bb8b63";

    let bytes: [u8; 20] = <[u8; 20]>::from_hex(contract_address_str).expect("Invalid hex");

    example(
        url::Url::from_str("wss://asset-hub-westend-rpc.n.dwellir.com").unwrap(),
        bytes.into(),
        "album enrich wink music lake angry nephew oven tomorrow gate brand undo",
    )
    .await;
}

async fn example(url: url::Url, contract_address: H160, signer_seed: &str) {
    let uri = <SecretUri as std::str::FromStr>::from_str(signer_seed).unwrap();
    let signer = Keypair::from_uri(&uri).unwrap();

    let opts = ExtrinsicOptsBuilder::<DefaultConfig, DefaultEnvironment, Keypair>::new(signer)
        .file("./metadata/vectest.contract".into())
        .url(url)
        .done();

    // let call_resutl: CallExec<DefaultConfig, DefaultEnvironment, Keypair> =
    let call_resutl = CallCommandBuilder::new(contract_address, "get_map_val", opts.clone())
        .args(vec!["101".to_string()])
        .done()
        .await;

    match call_resutl {
        Ok(call) => {
            // let call_result = call.call(None, 0.into()).await;
            let call_result = call.call_dry_run().await;
            println!("call_result: {:?}", call_result);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    };

    // let call_result = call.call(None, 0.into()).await;
    // println!("call_result: {:?}", call_result);
}

enum ContractError {}
