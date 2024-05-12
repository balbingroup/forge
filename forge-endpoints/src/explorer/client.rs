use anyhow::Result;
use forge_types::{block::{block::Block, block_transactions::BlockTransactions, header::Header}, transaction::transaction::Transaction};
use serde_json::Value;

#[derive(Clone, Debug)]
pub struct Client {
    pub ip: String,
}

impl Client {
    pub fn new(ip: &str) -> Self {
        Self {
            ip: ip.to_string(),
        }
    }

    pub async fn get_transaction_by_id(&self, id: &str) -> Result<Transaction> {
        let url = format!("{}/api/v1/transactions/{}", &self.ip, id);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<Transaction>().await?;
        Ok(response)
    }

}