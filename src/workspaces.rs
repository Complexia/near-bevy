use near_workspaces::error::Error;
use near_workspaces::{network::Sandbox, Contract, Worker};
#[derive(Default, Clone)]
pub struct NearInterface {
    pub contract: Option<Contract>,
}

impl NearInterface {
    pub async fn initialize_worker() -> Result<Worker<Sandbox>, Error> {
        let worker = near_workspaces::sandbox().await;
        return worker;
    }
}
