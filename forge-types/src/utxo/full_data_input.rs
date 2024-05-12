use serde::{Deserialize, Serialize};

use crate::asset::full_asset::FullAsset;

use super::additional_registers::AdditionalRegisters;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FullDataInput {
    #[serde(rename = "boxId")]
    pub box_id: String,
    pub value: u64,
    pub index: u32,
    #[serde(rename = "outputBlockId")]
    pub output_block_id: String,
    #[serde(rename = "outputTransactionId")]
    pub output_transaction_id: String,
    #[serde(rename = "outputIndex")]
    pub output_index: u32,
    #[serde(rename = "ergoTree")]
    pub ergo_tree: String,
    pub address: String,
    pub assets: Vec<FullAsset>,
    #[serde(rename = "additionalRegisters")]
    pub additional_registers: AdditionalRegisters,
}

impl FullDataInput {
    pub fn new(
        box_id: String,
        value: u64,
        index: u32,
        output_block_id: String,
        output_transaction_id: String,
        output_index: u32,
        ergo_tree: String,
        address: String,
        assets: Vec<FullAsset>,
        additional_registers: AdditionalRegisters,
    ) -> Self {
        Self {
            box_id,
            value,
            index,
            output_block_id,
            output_transaction_id,
            output_index,
            ergo_tree,
            address,
            assets,
            additional_registers,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_data_input() {
        let json = r#"{
            "boxId": "ceebde343d8e5f2224ca39416d4a529f557a32985ac39facf40395ce686f4356",
            "value": 16732143000000000,
            "index": 0,
            "outputBlockId": "94c54d8347c92a94b96e8245b1fb11cd7ba8e0cdc7e0a40c201e2064365cdf1f",
            "outputTransactionId": "6e1aa712aab3b6d31b5dc0471e1e1cee8157179195ebdce75e3cb978651c3eaa",
            "outputIndex": 0,
            "ergoTree": "101004020e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a7017300730110010204020404040004c0fd4f05808c82f5f6030580b8c9e5ae040580f882ad16040204c0944004c0f407040004000580f882ad16d19683030191a38cc7a7019683020193c2b2a57300007473017302830108cdeeac93a38cc7b2a573030001978302019683040193b1a5730493c2a7c2b2a573050093958fa3730673079973089c73097e9a730a9d99a3730b730c0599c1a7c1b2a5730d00938cc7b2a5730e0001a390c1a7730f",
            "address": "2Z4YBkDsDvQj8BX7xiySFewjitqp2ge9c99jfes2whbtKitZTxdBYqbrVZUvZvKv6aqn9by4kp3LE1c26LCyosFnVnm6b6U1JYvWpYmL2ZnixJbXLjWAWuBThV1D6dLpqZJYQHYDznJCk49g5TUiS4q8khpag2aNmHwREV7JSsypHdHLgJT7MGaw51aJfNubyzSKxZ4AJXFS27EfXwyCLzW1K6GVqwkJtCoPvrcLqmqwacAWJPkmh78nke9H4oT88XmSbRt2n9aWZjosiZCafZ4osUDxmZcc5QVEeTWn8drSraY3eFKe8Mu9MSCcVU",
            "assets": [
                {
                "tokenId": "20fa2bf23962cdf51b07722d6237c0c7b8a44f78856c0f7ec308dc1ef1a92a51",
                "index": 0,
                "amount": 1,
                "name": "Emission Contract NFT",
                "decimals": 0,
                "type": "EIP-004"
                },
                {
                "tokenId": "d9a2cc8a09abfaed87afacfbb7daee79a6b26f10c6613fc13d3f3953e5521d1a",
                "index": 1,
                "amount": 14170796000000000,
                "name": "Reemission Token",
                "decimals": 0,
                "type": "EIP-004"
                }
            ],
            "additionalRegisters": {
        
            }
        }"#;
        let data_input: FullDataInput = serde_json::from_str(json).unwrap();
        
        assert_eq!(data_input.box_id, "ceebde343d8e5f2224ca39416d4a529f557a32985ac39facf40395ce686f4356");
        assert_eq!(data_input.value, 16732143000000000);
        assert_eq!(data_input.address, "2Z4YBkDsDvQj8BX7xiySFewjitqp2ge9c99jfes2whbtKitZTxdBYqbrVZUvZvKv6aqn9by4kp3LE1c26LCyosFnVnm6b6U1JYvWpYmL2ZnixJbXLjWAWuBThV1D6dLpqZJYQHYDznJCk49g5TUiS4q8khpag2aNmHwREV7JSsypHdHLgJT7MGaw51aJfNubyzSKxZ4AJXFS27EfXwyCLzW1K6GVqwkJtCoPvrcLqmqwacAWJPkmh78nke9H4oT88XmSbRt2n9aWZjosiZCafZ4osUDxmZcc5QVEeTWn8drSraY3eFKe8Mu9MSCcVU");
    }
}