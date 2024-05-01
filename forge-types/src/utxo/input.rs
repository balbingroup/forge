use serde::{Deserialize, Serialize};

use super::spending_proof::SpendingProof;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Input {
    #[serde(rename = "boxId")]
    pub box_id: String,
    #[serde(rename = "spendingProof")]
    pub spending_proof: SpendingProof,
}

impl Input {
    pub fn new(box_id: String, spending_proof: SpendingProof) -> Self {
        Self {
            box_id,
            spending_proof,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let json = r#"{
            "boxId": "1ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117",
            "spendingProof": {
              "proofBytes": "4ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117",
              "extension": {
                "1": "a2aed72ff1b139f35d1ad2938cb44c9848a34d4dcfd6d8ab717ebde40a7304f2541cf628ffc8b5c496e6161eba3f169c6dd440704b1719e0"
              }
            }
        }"#;
    
        let input: Input = serde_json::from_str(json).unwrap();

        assert_eq!(
            input.box_id,
            "1ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117".to_string()
        );

        assert_eq!(
            input.spending_proof.proof_bytes,
            "4ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117".to_string()
        );
    }
}