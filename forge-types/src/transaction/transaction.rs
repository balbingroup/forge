use serde::{Deserialize, Serialize};

use crate::utxo::{data_input::DataInput, input::Input, output::Output};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub inputs: Vec<Input>,
    #[serde(rename = "dataInputs")]
    pub data_inputs: Vec<DataInput>,
    pub outputs: Vec<Output>,
    pub size: u64,
}

impl Transaction {
    pub fn new(
        id: String,
        inputs: Vec<Input>,
        data_inputs: Vec<DataInput>,
        outputs: Vec<Output>,
        size: u64,
    ) -> Self {
        Self {
            id,
            inputs,
            data_inputs,
            outputs,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction() {
        let json = r#"{
            "id": "4c6282be413c6e300a530618b37790be5f286ded758accc2aebd41554a1be308",
            "inputs": [
              {
                "boxId": "b69575e11c5c43400bfead5976ee0d6245a1168396b2e2a4f384691f275d501c",
                "spendingProof": {
                  "proofBytes": "",
                  "extension": {}
                }
              }
            ],
            "dataInputs": [],
            "outputs": [
              {
                "boxId": "71bc9534d4a4fe8ff67698a5d0f29782836970635de8418da39fee1cd964fcbe",
                "value": 93409065000000000,
                "ergoTree": "101004020e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a7017300730110010204020404040004c0fd4f05808c82f5f6030580b8c9e5ae040580f882ad16040204c0944004c0f407040004000580f882ad16d19683030191a38cc7a7019683020193c2b2a57300007473017302830108cdeeac93a38cc7b2a573030001978302019683040193b1a5730493c2a7c2b2a573050093958fa3730673079973089c73097e9a730a9d99a3730b730c0599c1a7c1b2a5730d00938cc7b2a5730e0001a390c1a7730f",
                "assets": [],
                "creationHeight": 1,
                "additionalRegisters": {},
                "transactionId": "4c6282be413c6e300a530618b37790be5f286ded758accc2aebd41554a1be308",
                "index": 0
              },
              {
                "boxId": "45dc27302332bcb93604ae63c0a543894b38af31e6aebdb40291e3e8ecaef031",
                "value": 67500000000,
                "ergoTree": "100204a00b08cd03be7ad70c74f691345cbedba19f4844e7fc514e1188a7929f5ae261d5bb00bb66ea02d192a39a8cc7a70173007301",
                "assets": [],
                "creationHeight": 1,
                "additionalRegisters": {},
                "transactionId": "4c6282be413c6e300a530618b37790be5f286ded758accc2aebd41554a1be308",
                "index": 1
              }
            ],
            "size": 341
          }"#;

        let transaction: Transaction = serde_json::from_str(json).unwrap();

        assert_eq!(transaction.id, "4c6282be413c6e300a530618b37790be5f286ded758accc2aebd41554a1be308");
        assert_eq!(transaction.size, 341);
    }
}