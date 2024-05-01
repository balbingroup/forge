use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalRegisters {
    #[serde(rename = "R4")]
    pub r4: Option<String>,
    #[serde(rename = "R5")]
    pub r5: Option<String>,
    #[serde(rename = "R6")]
    pub r6: Option<String>,
    #[serde(rename = "R7")]
    pub r7: Option<String>,
    #[serde(rename = "R8")]
    pub r8: Option<String>,
    #[serde(rename = "R9")]
    pub r9: Option<String>,
}

impl AdditionalRegisters {
    pub fn new(
        r4: Option<String>,
        r5: Option<String>,
        r6: Option<String>,
        r7: Option<String>,
        r8: Option<String>,
        r9: Option<String>,
    ) -> Self {
        Self {
            r4,
            r5,
            r6,
            r7,
            r8,
            r9,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_additional_registers() {
        let json = r#"{
            "R4": "0e240008cd0272eef9f3de497d5f76e15c9d7f8a91bfc87610899196f8677989876e83e354d1",
            "R5": "0e240008cd0272eef9f3de497d5f76e15c9d7f8a91bfc87610899196f8677989876e83e354d1",
            "R6": "110280c8afa02580a8d6b907",
            "R7": "0580a7c39dfc60",
            "R8": "058090dfc04a",
            "R9": "0e697b22696e697469616c426964223a353030303030303030302c22737461727454696d65223a313636353334333839353133342c226465736372697074696f6e223a224379626572436170796261726120233320437962657270756e6b20636f6c6c656374696f6e227d"
        }"#;

        let additional_registers: AdditionalRegisters = serde_json::from_str(json).unwrap();

        assert_eq!(
            additional_registers.r4,
            Some("0e240008cd0272eef9f3de497d5f76e15c9d7f8a91bfc87610899196f8677989876e83e354d1".to_string())
        );

        assert_eq!(
            additional_registers.r7,
            Some("0580a7c39dfc60".to_string())
        );
    }
}