use bevy::prelude::{App, Component, Plugin, System};
use near_units::parse_near;
use near_workspaces::{Account, Contract, DevNetwork, Worker};
use serde_json::json;

use crate::systems::{add_people, greet_people};

#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub String);

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_people).add_system(greet_people);
    }
}

/// Wraps the default bevy AssetIo and adds support for loading http urls
pub struct NearInterface;

impl NearInterface {
    //provide WASM filepath for the NFT and mint it
    async fn mint_nft(nft_wasm_filepath: &str) {
        let worker = near_workspaces::sandbox().await.unwrap();
        let wasm = std::fs::read(nft_wasm_filepath).unwrap();
        let contract = worker.dev_deploy(&wasm).await.unwrap();

        let outcome = contract
            .call("new_default_meta")
            .args_json(json!({
                "owner_id": contract.id(),
            }))
            .transact() // note: we use the contract's keys here to sign the transaction
            .await
            .unwrap();

        // outcome contains data like logs, receipts and transaction outcomes.
        println!("new_default_meta outcome: {:#?}", outcome);

        let deposit = 10000000000000000000000;
        let outcome = contract
            .call("nft_mint")
            .args_json(json!({
                "token_id": "0",
                "token_owner_id": contract.id(),
                "token_metadata": {
                    "title": "Olympus Mons",
                    "dscription": "Tallest mountain in charted solar system",
                    "copies": 1,
                },
            }))
            .deposit(deposit)
            // nft_mint might consume more than default gas, so supply our own gas value:
            .gas(near_units::parse_gas!("300 T") as u64)
            .transact()
            .await
            .unwrap();

        println!("nft_mint outcome: {:#?}", outcome);

        let result: serde_json::Value = contract
            .call("nft_metadata")
            .view()
            .await
            .unwrap()
            .json()
            .unwrap();

        println!("--------------\n{}", result);
        println!("Dev Account ID: {}", contract.id());
    }

    //execure a function of a contract and print outcome
    async fn execute_contract_function(
        contract: &Contract,
        contract_function: &str,
        msg: &str,
    ) -> anyhow::Result<()> {
        // Call into the function `contract_function` with args:
        let result: serde_json::Value = contract
            .call(contract_function)
            .args_json(serde_json::json!({
                "message": msg,
            }))
            .transact()
            .await?
            .json()?;

        println!("--------------\n{}", result);
        println!("Dev Account ID: {}", contract.id());

        Ok(())
    }
    //pass account as arg and get its balance
    async fn get_account_balance(account: Account) -> anyhow::Result<()> {
        //let worker = near_workspaces::sandbox().await?;
        //create account on the spot for testing
        //let dev_account = worker.dev_create_account().await?;

        let balance = account.view_account().await?.balance;
        println!("Dev account's balance is: {}", balance);

        Ok(())
    }
}
