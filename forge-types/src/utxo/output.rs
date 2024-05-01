use serde::{Deserialize, Serialize};

use crate::asset::asset::Asset;

use super::additional_registers::AdditionalRegisters;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Output {
    #[serde(rename = "boxId")]
    pub box_id: String,
    pub value: u64,
    #[serde(rename = "ergoTree")]
    pub ergo_tree: String,
    #[serde(rename = "creationHeight")]
    pub creation_height: u64,
    pub assets: Vec<Asset>,
    #[serde(rename = "additionalRegisters")]
    pub additional_registers: AdditionalRegisters,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    pub index: u64,
}

impl Output {
    pub fn new(
        box_id: String,
        value: u64,
        ergo_tree: String,
        creation_height: u64,
        assets: Vec<Asset>,
        additional_registers: AdditionalRegisters,
        transaction_id: String,
        index: u64,
    ) -> Self {
        Self {
            box_id,
            value,
            ergo_tree,
            creation_height,
            assets,
            additional_registers,
            transaction_id,
            index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let json = r#"{
            "boxId": "1ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117",
            "value": 147,
            "ergoTree": "0008cd0336100ef59ced80ba5f89c4178ebd57b6c1dd0f3d135ee1db9f62fc634d637041",
            "creationHeight": 9149,
            "assets": [
              {
                "tokenId": "4ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117",
                "amount": 1000
              }
            ],
            "additionalRegisters": {
              "R4": "100204a00b08cd0336100ef59ced80ba5f89c4178ebd57b6c1dd0f3d135ee1db9f62fc634d637041ea02d192a39a8cc7a70173007301"
            },
            "transactionId": "2ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117",
            "index": 0
          }"#;
        
        let output: Output = serde_json::from_str(json).unwrap();

        assert_eq!(
            output.box_id,
            "1ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117".to_string()
        );

        assert_eq!(output.value, 147);
        assert_eq!(
            output.ergo_tree,
            "0008cd0336100ef59ced80ba5f89c4178ebd57b6c1dd0f3d135ee1db9f62fc634d637041".to_string()
        );
        assert_eq!(output.assets.get(0).unwrap().token_id, "4ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117");
    }
}