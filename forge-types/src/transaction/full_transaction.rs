use serde::{Deserialize, Serialize};

use crate::utxo::{full_data_input::FullDataInput, full_input::FullInput, full_output::FullOutput};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullTransaction {
    pub id: String,
    #[serde(rename = "blockId")]
    pub block_id: String,
    #[serde(rename = "inclusionHeight")]
    pub inclusion_height: u32,
    pub timestamp: u64,
    pub index: u32,
    #[serde(rename = "globalIndex")]
    pub global_index: u32,
    #[serde(rename = "numConfirmations")]
    pub num_confirmations: u32,
    pub inputs: Vec<FullInput>,
    #[serde(rename = "dataInputs")]
    pub data_inputs: Vec<FullDataInput>,
    pub outputs: Vec<FullOutput>,
    pub size: u32,
}

impl FullTransaction {
    pub fn new(
        id: String,
        block_id: String,
        inclusion_height: u32,
        timestamp: u64,
        index: u32,
        global_index: u32,
        num_confirmations: u32,
        inputs: Vec<FullInput>,
        data_inputs: Vec<FullDataInput>,
        outputs: Vec<FullOutput>,
        size: u32,
    ) -> Self {
        Self {
            id,
            block_id,
            inclusion_height,
            timestamp,
            index,
            global_index,
            num_confirmations,
            inputs,
            data_inputs,
            outputs,
            size,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_transaction() {
        let json = r#"{
            "id": "1e9df28131e9809ce07f93317e613397d8fce85289427f183ee4a3372cd1734e",
            "blockId": "4f07f05ad6acd4cb9146e4afe940c78c831bcff6aae41dbc796a65c1435d82c7",
            "inclusionHeight": 1262991,
            "timestamp": 1715538858186,
            "index": 0,
            "globalIndex": 1031936,
            "numConfirmations": 2,
            "inputs": [
                {
                "boxId": "ceebde343d8e5f2224ca39416d4a529f557a32985ac39facf40395ce686f4356",
                "value": 16732143000000000,
                "index": 0,
                "spendingProof": null,
                "outputBlockId": "94c54d8347c92a94b96e8245b1fb11cd7ba8e0cdc7e0a40c201e2064365cdf1f",
                "outputTransactionId": "6e1aa712aab3b6d31b5dc0471e1e1cee8157179195ebdce75e3cb978651c3eaa",
                "outputIndex": 0,
                "outputGlobalIndex": 5542282,
                "outputCreatedAt": 1262990,
                "outputSettledAt": 1262990,
                "ergoTree": "101004020e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a7017300730110010204020404040004c0fd4f05808c82f5f6030580b8c9e5ae040580f882ad16040204c0944004c0f407040004000580f882ad16d19683030191a38cc7a7019683020193c2b2a57300007473017302830108cdeeac93a38cc7b2a573030001978302019683040193b1a5730493c2a7c2b2a573050093958fa3730673079973089c73097e9a730a9d99a3730b730c0599c1a7c1b2a5730d00938cc7b2a5730e0001a390c1a7730f",
                "ergoTreeConstants": "0: 1\n1: Coll(16,2,4,-96,11,8,-51,2,121,-66,102,126,-7,-36,-69,-84,85,-96,98,-107,-50,-121,11,7,2,-101,-4,-37,45,-50,40,-39,89,-14,-127,91,22,-8,23,-104,-22,2,-47,-110,-93,-102,-116,-57,-89,1,115,0,115,1)\n2: Coll(1)\n3: 1\n4: 2\n5: 0\n6: 655200\n7: 67500000000\n8: 75000000000\n9: 3000000000\n10: 1\n11: 525600\n12: 64800\n13: 0\n14: 0\n15: 3000000000",
                "ergoTreeScript": "{sigmaProp(\n  allOf(\n    Coll[Boolean](\n      HEIGHT \u003E SELF.creationInfo._1, allOf(\n        Coll[Boolean](\n          OUTPUTS(placeholder[Int](0)).propositionBytes == substConstants(\n            placeholder[Coll[Byte]](1), placeholder[Coll[Int]](2), Coll[SigmaProp](proveDlog(decodePoint(minerPubKey)))\n          ), HEIGHT == OUTPUTS(placeholder[Int](3)).creationInfo._1\n        )\n      ), anyOf(\n        Coll[Boolean](\n          allOf(\n            Coll[Boolean](\n              OUTPUTS.size == placeholder[Int](4), SELF.propositionBytes == OUTPUTS(placeholder[Int](5)).propositionBytes, if (HEIGHT \u003C placeholder[Int](6)) {\n                placeholder[Long](7)\n              } else {\n                placeholder[Long](8) - placeholder[Long](9) * placeholder[Int](10) + HEIGHT - placeholder[Int](11) / placeholder[Int](12).toLong\n              } == SELF.value - OUTPUTS(placeholder[Int](13)).value, OUTPUTS(placeholder[Int](14)).creationInfo._1 == HEIGHT\n            )\n          ), SELF.value \u003C= placeholder[Long](15)\n        )\n      )\n    )\n  )\n)}",
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
                }
            ],
            "dataInputs": [],
            "outputs": [
                {
                "boxId": "ca46cd712cd5b55d5c6df3888b742623f82a3573b44608b3eba2adb01dfd4879",
                "transactionId": "1e9df28131e9809ce07f93317e613397d8fce85289427f183ee4a3372cd1734e",
                "blockId": "4f07f05ad6acd4cb9146e4afe940c78c831bcff6aae41dbc796a65c1435d82c7",
                "value": 16732104000000000,
                "index": 0,
                "globalIndex": 5542288,
                "creationHeight": 1262991,
                "settlementHeight": 1262991,
                "ergoTree": "101004020e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a7017300730110010204020404040004c0fd4f05808c82f5f6030580b8c9e5ae040580f882ad16040204c0944004c0f407040004000580f882ad16d19683030191a38cc7a7019683020193c2b2a57300007473017302830108cdeeac93a38cc7b2a573030001978302019683040193b1a5730493c2a7c2b2a573050093958fa3730673079973089c73097e9a730a9d99a3730b730c0599c1a7c1b2a5730d00938cc7b2a5730e0001a390c1a7730f",
                "ergoTreeConstants": "0: 1\n1: Coll(16,2,4,-96,11,8,-51,2,121,-66,102,126,-7,-36,-69,-84,85,-96,98,-107,-50,-121,11,7,2,-101,-4,-37,45,-50,40,-39,89,-14,-127,91,22,-8,23,-104,-22,2,-47,-110,-93,-102,-116,-57,-89,1,115,0,115,1)\n2: Coll(1)\n3: 1\n4: 2\n5: 0\n6: 655200\n7: 67500000000\n8: 75000000000\n9: 3000000000\n10: 1\n11: 525600\n12: 64800\n13: 0\n14: 0\n15: 3000000000",
                "ergoTreeScript": "{sigmaProp(\n  allOf(\n    Coll[Boolean](\n      HEIGHT \u003E SELF.creationInfo._1, allOf(\n        Coll[Boolean](\n          OUTPUTS(placeholder[Int](0)).propositionBytes == substConstants(\n            placeholder[Coll[Byte]](1), placeholder[Coll[Int]](2), Coll[SigmaProp](proveDlog(decodePoint(minerPubKey)))\n          ), HEIGHT == OUTPUTS(placeholder[Int](3)).creationInfo._1\n        )\n      ), anyOf(\n        Coll[Boolean](\n          allOf(\n            Coll[Boolean](\n              OUTPUTS.size == placeholder[Int](4), SELF.propositionBytes == OUTPUTS(placeholder[Int](5)).propositionBytes, if (HEIGHT \u003C placeholder[Int](6)) {\n                placeholder[Long](7)\n              } else {\n                placeholder[Long](8) - placeholder[Long](9) * placeholder[Int](10) + HEIGHT - placeholder[Int](11) / placeholder[Int](12).toLong\n              } == SELF.value - OUTPUTS(placeholder[Int](13)).value, OUTPUTS(placeholder[Int](14)).creationInfo._1 == HEIGHT\n            )\n          ), SELF.value \u003C= placeholder[Long](15)\n        )\n      )\n    )\n  )\n)}",
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
                    "amount": 14170784000000000,
                    "name": "Reemission Token",
                    "decimals": 0,
                    "type": "EIP-004"
                    }
                ],
                "additionalRegisters": {
            
                },
                "spentTransactionId": "3c33f24ce7a211a8af08ca12522c3721f46dd2fc814ac37f4571f7d7d5115c1f",
                "mainChain": true
                },
                {
                "boxId": "f9f2c7deab68392371413c983c59049b999b3f5af51b461250215aa5351315f2",
                "transactionId": "1e9df28131e9809ce07f93317e613397d8fce85289427f183ee4a3372cd1734e",
                "blockId": "4f07f05ad6acd4cb9146e4afe940c78c831bcff6aae41dbc796a65c1435d82c7",
                "value": 39000000000,
                "index": 1,
                "globalIndex": 5542289,
                "creationHeight": 1262991,
                "settlementHeight": 1262991,
                "ergoTree": "100204a00b08cd02eeec374f4e660e117fccbfec79e6fe5cdf44ac508fa228bfc654d2973f9bdc9aea02d192a39a8cc7a70173007301",
                "ergoTreeConstants": "0: 720\n1: SigmaProp(ProveDlog(ECPoint(eeec37,211bb1,...)))",
                "ergoTreeScript": "{sigmaProp(HEIGHT \u003E= SELF.creationInfo._1 + placeholder[Int](0)) && placeholder[SigmaProp](1)}",
                "address": "88dhgzEuTXaSuf5QC1TJDgdxqJMQEQAM6YaTTRqmUDrmPoVky1b16WAK5zMrq3p2mYqpUNKCyi5CLS9V",
                "assets": [
                    {
                    "tokenId": "d9a2cc8a09abfaed87afacfbb7daee79a6b26f10c6613fc13d3f3953e5521d1a",
                    "index": 0,
                    "amount": 12000000000,
                    "name": "Reemission Token",
                    "decimals": 0,
                    "type": "EIP-004"
                    }
                ],
                "additionalRegisters": {
            
                },
                "spentTransactionId": null,
                "mainChain": true
                }
            ],
            "size": 425
        }"#;

        let transaction: FullTransaction = serde_json::from_str(json).unwrap();

        assert_eq!(transaction.id, "1e9df28131e9809ce07f93317e613397d8fce85289427f183ee4a3372cd1734e");
        assert_eq!(transaction.inputs.get(0).unwrap().box_id, "ceebde343d8e5f2224ca39416d4a529f557a32985ac39facf40395ce686f4356");
        assert_eq!(transaction.data_inputs.len(), 0);
        assert_eq!(transaction.outputs.get(0).unwrap().box_id, "ca46cd712cd5b55d5c6df3888b742623f82a3573b44608b3eba2adb01dfd4879");
        assert_eq!(transaction.size, 425);
    }
}