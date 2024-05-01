use serde::{Deserialize, Serialize};

use super::key_value_item::KeyValueItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockExtension {
    #[serde(rename = "headerId")]
    pub header_id: String,
    pub fields: Vec<KeyValueItem>,
    pub digest: String,
}

impl BlockExtension {
    pub fn new(header_id: String, fields: Vec<KeyValueItem>, digest: String) -> Self {
        Self {
            header_id,
            fields,
            digest,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_extension() {
        let json = r#"{
            "headerId": "9fce9bcc768a2d9231cc4acad51ccb1bb05f2c5b43f08b7893dda13af0ef2b54",
            "digest": "cc5ccdcc344a7ffaaadcfe7422607224f8ba1a6913c9d405e5dcf32af4759e6e",
            "fields": [
                [
                    "0100",
                    "01b0244dfc267baca974a4caee06120321562784303a8a688976ae56170e4d175b"
                ],
                [
                    "0101",
                    "031155d54de65f0130fae142aa4cf5a7728b7c30f5939d33fddf077e2008040a15"
                ],
                [
                    "0104",
                    "01bf770872186c2f6dcc8f765b4732937592d358c2a585f64c3d7f73f703a666a0"
                ],
                [
                    "0105",
                    "01636c920aea07d2d2da356ac8316bed28078c238ee5784447408b80526a03fbc8"
                ],
                [
                    "0106",
                    "06673051716c2594922eb16be081fc68032219065cc64aeee79da613bd4894df14"
                ],
                [
                    "010c",
                    "01ac8230cf607dccdb465917096d12aa608225618857bb302f399e5dbd2da59b64"
                ],
                [
                    "010d",
                    "01930ae449fcec6943e43d95133de9c72b309d270947d5a98c999b664c168672e0"
                ],
                [
                    "010e",
                    "02ac9a1057d0e32ff4bdfed54c0010215c89aa7c69d664f9aa69f148390390d410"
                ],
                [
                    "0110",
                    "032ae1895b21f4c84999b9e25c0a311629e372c235ab2ddf54f98647ef5b74e0ed"
                ],
                [
                    "0113",
                    "015dc3126f2785e72c8d371d449c4b2220942b77161a55c3d91d6bd02bfb7bf4a4"
                ],
                [
                    "0114",
                    "01fd23b031b9476e1b9420ccfa578844f769ddc45d8b28b963dce8d02456e45375"
                ]
            ]
        }"#;

        let block_extension: BlockExtension = serde_json::from_str(json).unwrap();
        let header_id = "9fce9bcc768a2d9231cc4acad51ccb1bb05f2c5b43f08b7893dda13af0ef2b54";
        let digest = "cc5ccdcc344a7ffaaadcfe7422607224f8ba1a6913c9d405e5dcf32af4759e6e";

        assert_eq!(block_extension.header_id, header_id);
        assert_eq!(block_extension.digest, digest);
        assert_eq!(block_extension.fields.len(), 11);
        assert_eq!(block_extension.fields[6].value, "01930ae449fcec6943e43d95133de9c72b309d270947d5a98c999b664c168672e0");
    }
}
