use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    #[serde(rename = "tokenId")]
    pub token_id: String,
    pub amount: u64,
}

impl Asset {
    pub fn new(token_id: &str, amount: u64) -> Self {
        Self {
            token_id: token_id.to_string(),
            amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset() {
        let json = r#"{
            "tokenId": "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51",
            "amount": 1
        }"#;

        let asset: Asset = serde_json::from_str(json).unwrap();
        let token_id = "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51";
        let amount = 1;

        assert_eq!(asset.token_id, token_id);
        assert_eq!(asset.amount, amount);
    }

    #[test]
    fn test_multiple_assets() {
        let json = r#"[{
            "tokenId": "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51",
            "amount": 1
        }, {
            "tokenId": "d9a2cc8a09abfaed87afacfbb7daee79a6b26f10c6613fc13d3f3953e5521d1a",
            "amount": 14270732000000000
        }]"#;

        let assets: Vec<Asset> = serde_json::from_str(json).unwrap();
        let token_id1 = "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51";
        let amount1 = 1;
        let token_id2 = "d9a2cc8a09abfaed87afacfbb7daee79a6b26f10c6613fc13d3f3953e5521d1a";
        let amount2 = 14270732000000000;

        assert_eq!(assets[0].token_id, token_id1);
        assert_eq!(assets[0].amount, amount1);
        assert_eq!(assets[1].token_id, token_id2);
        assert_eq!(assets[1].amount, amount2);
    }
}