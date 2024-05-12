use anyhow::Result;
use forge_types::transaction::full_transaction::FullTransaction;

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

    pub async fn get_transaction_by_id(&self, id: &str) -> Result<FullTransaction> {
        let url = format!("{}/api/v1/transactions/{}", &self.ip, id);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<FullTransaction>().await?;
        Ok(response)
    }

}