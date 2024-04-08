use std::{fs::File, path::Path, str::FromStr};

use fuels::{
    crypto::SecretKey,
    prelude::*,
    types::{Bits256, Identity},
};

use anyhow::Result;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // let provider = Provider::connect("beta-5.fuel.network").await.unwrap();
    let server = FuelService::start(Config::default()).await?;

    // Create a client that will talk to the node created above.
    let provider = Provider::from(server.bound_address()).await?;

    let secret = SecretKey::from_str(&env::var("PRIVATE_KEY").unwrap()).unwrap();

    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));

    let wallet_address = wallet.address().clone();

    let contract_address =
        ContractId::from_str("0x0eef758208bfbff4f23353cc35913e0bf7ee18b06e1af57e64eb86cbea5ce6a7")
            .unwrap();

    let contract_id = Bech32ContractId::from(contract_address);

    abigen!(Contract(
        name = "NFTContract",
        abi = "NFT-contract/out/debug/NFT-contract-abi.json"
    ));

    // This is an instance of your contract which you can use to make calls to your functions
    let contract_instance = NFTContract::new(contract_id, wallet);

    let recipient = Identity::Address(wallet_address.into());
    let sub_id =
        Bits256::from_hex_str("0x0eef758208bfbff4f23353cc35913e0bf7ee18b06e1af57e64eb86cbea5ce6a7")
            .unwrap();

    let response = contract_instance
        .methods()
        .mint(recipient, sub_id, 1)
        .call()
        .await?;

    println!(
        "Produce log variables called at tx_id: {}",
        response.tx_id.unwrap()
    );

    Ok(())
}
