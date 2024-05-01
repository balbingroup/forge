use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Parameters {
    pub height: u32,
    #[serde(rename = "storageFeeFactor")]
    pub storage_fee_factor: u32,
    #[serde(rename = "minValuePerByte")]
    pub min_value_per_byte: u32,
    #[serde(rename = "maxBlockSize")]
    pub max_block_size: u32,
    #[serde(rename = "maxBlockCost")]
    pub max_block_cost: u32,
    #[serde(rename = "blockVersion")]
    pub block_version: u32,
    #[serde(rename = "tokenAccessCost")]
    pub token_access_cost: u32,
    #[serde(rename = "inputCost")]
    pub input_cost: u32,
    #[serde(rename = "dataInputCost")]
    pub data_input_cost: u32,
    #[serde(rename = "outputCost")]
    pub output_cost: u32,
}