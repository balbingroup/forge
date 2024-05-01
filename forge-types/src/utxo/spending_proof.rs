use serde::{Deserialize, Serialize};

use super::input_extension::InputExtension;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendingProof {
    #[serde(rename = "proofBytes")]
    pub proof_bytes: String,
    pub extension: InputExtension,
}

impl SpendingProof {
    pub fn new(proof_bytes: String, extension: InputExtension) -> Self {
        Self {
            proof_bytes,
            extension,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spending_proof() {
        let json = r#"{
            "proofBytes": "4ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117",
            "extension": {
              "1": "a2aed72ff1b139f35d1ad2938cb44c9848a34d4dcfd6d8ab717ebde40a7304f2541cf628ffc8b5c496e6161eba3f169c6dd440704b1719e0"
            }
          }"#;
        
        let spending_proof: SpendingProof = serde_json::from_str(json).unwrap();

        assert_eq!(
            spending_proof.proof_bytes,
            "4ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd1173ab9da11fc216660e974842cc3b7705e62ebb9e0bf5ff78e53f9cd40abadd117".to_string()
        );
        assert_eq!(
            spending_proof.extension.extension.get("1").unwrap(),
            "a2aed72ff1b139f35d1ad2938cb44c9848a34d4dcfd6d8ab717ebde40a7304f2541cf628ffc8b5c496e6161eba3f169c6dd440704b1719e0"
        );
    }
}