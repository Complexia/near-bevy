use near_workspaces::types::SecretKey;
use near_workspaces::AccountId;
use near_workspaces::Contract;

#[derive(Default, Clone)]
pub struct NearInterface {
    pub contract: Option<Contract>,
}

impl NearInterface {
    pub fn initialize_contract(
        contract_account: &str,
        account_id: &str,
        secret_key: SecretKey,
        block_height: i32,
    ) {
        let contract_account_id: AccountId = "alice.near".parse().unwrap();
        //return contract
    }
}
