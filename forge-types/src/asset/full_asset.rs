use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullAsset {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    pub index: u32,
    pub amount: u64,
    pub name: String,
    pub decimals: u32,
    #[serde(rename = "type")]
    pub asset_type: String,
}

impl FullAsset {
    pub fn new(
        token_id: String,
        index: u32,
        amount: u64,
        name: String,
        decimals: u32,
        asset_type: String,
    ) -> Self {
        Self {
            token_id,
            index,
            amount,
            name,
            decimals,
            asset_type,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_asset() {
        let json = r#"{
            "tokenId": "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51",
            "index": 0,
            "amount": 1,
            "name": "Emission Contract NFT",
            "decimals": 0,
            "type": "EIP-004"
        }"#;
        let asset: FullAsset = serde_json::from_str(json).unwrap();
        assert_eq!(asset.token_id, "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51");
        assert_eq!(asset.index, 0);
        assert_eq!(asset.amount, 1);
        assert_eq!(asset.name, "Emission Contract NFT");
        assert_eq!(asset.decimals, 0);
        assert_eq!(asset.asset_type, "EIP-004");
    }
}