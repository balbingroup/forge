use anyhow::Result;
use ergo_types::block::{block::Block, block_transactions::BlockTransactions, header::Header};
use serde_json::Value;

use crate::types::{indexed_height_response::IndexedHeightResponse, node_info::NodeInfo, peer::Peer};

#[derive(Clone, Debug)]
pub struct Client {
    pub ip: String,
    pub port: u32,
    pub api_key: String,
}

impl Client {
    pub fn new(ip: &str, port: u32, api_key: &str) -> Self {
        Self {
            ip: ip.to_string(),
            port,
            api_key: api_key.to_string(),
        }
    }

    pub async fn get_block_ids(&self, limit: u32, offset: u32) -> Result<Vec<String>> {
        let url = format!("http://{}:{}/blocks?limit={}&offset={}", &self.ip, &self.port, limit, offset);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        let response = response.as_array().unwrap();
        let response = response.iter().map(|x| x.as_str().unwrap().to_string()).collect();
        Ok(response)
    }

    pub async fn get_block_id_for_height(&self, height: u64) -> Result<String> {
        let url = format!("http://{}:{}/blocks/at/{}", &self.ip, &self.port, height);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        let response = response[0].as_str().unwrap();
        Ok(response.to_string())
    }

    pub async fn get_chain_slice(&self, from: u32, to: u32) -> Result<Vec<Header>> {
        let url = format!("http://{}:{}/blocks/chainSlice?fromHeight={}&toHeight={}", &self.ip, &self.port, from, to);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        let response = response.as_array().unwrap();
        let response = response.iter().map(|x| serde_json::from_value(x.clone()).unwrap()).collect();
        Ok(response)
    }
    
    pub async fn get_block_by_id(&self, id: &str) -> Result<Block> {
        let url = format!("http://{}:{}/blocks/{}", &self.ip, &self.port, id);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<Block>().await?;
        Ok(response)
    }

    pub async fn get_header_by_block_id(&self, id: &str) -> Result<Header> {
        let url = format!("http://{}:{}/blocks/{}/header", &self.ip, &self.port, id);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        let response = serde_json::from_value(response).unwrap();
        Ok(response)
    }

    pub async fn get_transactions_by_block_id(&self, id: &str) -> Result<BlockTransactions> {
        let url = format!("http://{}:{}/blocks/{}/transactions", &self.ip, &self.port, id);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        let response = serde_json::from_value(response).unwrap();
        Ok(response)
    }

    pub async fn get_merkle_proof_for_transaction(&self, block_id: &str, tx_id: &str) -> Result<Value> {
        let url = format!("http://{}:{}/blocks/{}/proofFor/{}", &self.ip, &self.port, block_id, tx_id);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        Ok(response)
    }

    pub async fn get_last_headers(&self, count: u32) -> Result<Vec<Header>> {
        let url = format!("http://{}:{}/blocks/lastHeaders/{}", &self.ip, &self.port, count);
        let response = reqwest::get(&url).await?.text().await?;
        let response: Value = serde_json::from_str(&response)?;
        let response = response.as_array().unwrap();
        let response = response.iter().map(|x| serde_json::from_value(x.clone()).unwrap()).collect();
        Ok(response)
    }

    pub async fn get_node_info(&self) -> Result<NodeInfo> {
        let url = format!("http://{}:{}/info", &self.ip, &self.port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<NodeInfo>().await?;
        Ok(response)
    }

    pub async fn get_all_peers(&self) -> Result<Vec<Peer>> {
        let url = format!("http://{}:{}/peers/all", &self.ip, &self.port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<Vec<Peer>>().await?;
        Ok(response)
    }

    pub async fn get_connected_peers(&self) -> Result<Vec<Peer>> {
        let url = format!("http://{}:{}/peers/connected", &self.ip, &self.port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<Vec<Peer>>().await?;
        Ok(response)
    }

    pub async fn get_blacklisted_peers(&self) -> Result<Vec<String>> {
        let url = format!("http://{}:{}/peers/blacklisted", &self.ip, &self.port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<Value>().await?;
        let response = response["addresses"].as_array().unwrap();
        let response = response.iter().map(|x| x.as_str().unwrap().to_string()).collect();
        Ok(response)
    }

    pub async fn get_indexed_height(&self) -> Result<IndexedHeightResponse> {
        let url = format!("http://{}:{}/blockchain/indexedHeight", &self.ip, &self.port);
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?.json::<IndexedHeightResponse>().await?;
        Ok(response)
    }
}