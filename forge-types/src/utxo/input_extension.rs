use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputExtension {
    #[serde(flatten)]
    pub extension: HashMap<String, String>
}

impl InputExtension {
    pub fn new(extension: HashMap<String, String>) -> Self {
        Self { extension }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_extension() {
        let json = r#"{
            "1": "a2aed72ff1b139f35d1ad2938cb44c9848a34d4dcfd6d8ab717ebde40a7304f2541cf628ffc8b5c496e6161eba3f169c6dd440704b1719e0"
        }"#;

        let input_extension: InputExtension = serde_json::from_str(json).unwrap();

        assert_eq!(
            input_extension.extension.get("1").unwrap(),
            "a2aed72ff1b139f35d1ad2938cb44c9848a34d4dcfd6d8ab717ebde40a7304f2541cf628ffc8b5c496e6161eba3f169c6dd440704b1719e0"
        );
    }
}
