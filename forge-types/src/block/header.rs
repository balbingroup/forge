use ergo_chain_types::{AutolykosSolution, Votes};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub version: u8,
    pub id: String,
    #[serde(rename = "parentId")]
    pub parent_id: String,
    #[serde(rename = "adProofsRoot")]
    pub ad_proofs_root: String,
    #[serde(rename = "stateRoot")]
    pub state_root: String,
    #[serde(rename = "transactionsRoot")]
    pub transaction_root: String,
    pub timestamp: u64,
    #[serde(rename = "nBits")]
    pub n_bits: u64,
    pub height: u32,
    #[serde(rename = "extensionHash")]
    pub extension_root: String,
    #[serde(rename = "powSolutions")]
    pub autolykos_solution: AutolykosSolution,
    pub difficulty: String,
    #[serde(rename = "extensionId")]
    pub extension_id: String,
    #[serde(rename = "transactionsId")]
    pub transactions_id: String,
    #[serde(rename = "adProofsId")]
    pub ad_proofs_id: String,
    pub votes: Votes,
    pub size: u64,
}

impl Header {
    pub fn new(
        version: u8,
        id: String,
        parent_id: String,
        ad_proofs_root: String,
        state_root: String,
        transaction_root: String,
        timestamp: u64,
        n_bits: u64,
        height: u32,
        extension_root: String,
        autolykos_solution: AutolykosSolution,
        difficulty: String,
        extension_id: String,
        transactions_id: String,
        ad_proofs_id: String,
        votes: Votes,
        size: u64,
    ) -> Self {
        Self {
            version,
            id,
            parent_id,
            ad_proofs_root,
            state_root,
            transaction_root,
            timestamp,
            n_bits,
            height,
            extension_root,
            autolykos_solution,
            difficulty,
            extension_id,
            transactions_id,
            ad_proofs_id,
            votes,
            size,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let json = r#"{
                "extensionId": "af4c9de8106960b47964d21e6eb2acdad7e3e168791e595f0806ebfb036ee7de",
                "difficulty": "1199990374400",
                "votes": "000000",
                "timestamp": 1561978977137,
                "size": 279,
                "stateRoot": "18b7a08878f2a7ee4389c5a1cece1e2724abe8b8adc8916240dd1bcac069177303",
                "height": 1,
                "nBits": 100734821,
                "version": 1,
                "id": "b0244dfc267baca974a4caee06120321562784303a8a688976ae56170e4d175b",
                "adProofsRoot": "766ab7a313cd2fb66d135b0be6662aa02dfa8e5b17342c05a04396268df0bfbb",
                "transactionsRoot": "93fb06aa44413ff57ac878fda9377207d5db0e78833556b331b4d9727b3153ba",
                "extensionHash": "0e5751c026e543b2e8ab2eb06099daa1d1e5df47778f7787faab45cdf12fe3a8",
                "powSolutions": {
                    "pk": "03be7ad70c74f691345cbedba19f4844e7fc514e1188a7929f5ae261d5bb00bb66",
                    "w": "02da9385ac99014ddcffe88d2ac5f28ce817cd615f270a0a5eae58acfb9fd9f6a0",
                    "n": "000000030151dc63",
                    "d": 46909460813884299753486408728361968139945651324239558400157099627
                },
                "adProofsId": "cfc4af9743534b30ef38deec118a85ce6f0a3741b79b7d294f3e089c118188dc",
                "transactionsId": "fc13e7fd2d1ddbd10e373e232814b3c9ee1b6fbdc4e6257c288ecd9e6da92633",
                "parentId": "0000000000000000000000000000000000000000000000000000000000000000"
          }"#;

        let header: Header = serde_json::from_str(json).unwrap();

        assert_eq!(header.size, 279);
        assert_eq!(header.id, "b0244dfc267baca974a4caee06120321562784303a8a688976ae56170e4d175b");
        assert_eq!(header.ad_proofs_id, "cfc4af9743534b30ef38deec118a85ce6f0a3741b79b7d294f3e089c118188dc");
    }
}