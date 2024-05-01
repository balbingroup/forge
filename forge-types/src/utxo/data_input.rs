use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataInput {
    #[serde(rename = "boxId")]
    pub box_id: String,
}

impl DataInput {
    pub fn new(box_id: String) -> Self {
        Self { box_id }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_input() {
        let json = r#"{
            "boxId": "9d8f67920f7bc962db574f105430b658c8c4791d8b1ee6e158927035b0373d3a"
        }"#;

        let data_input: DataInput = serde_json::from_str(json).unwrap();

        assert_eq!(
            data_input.box_id,
            "9d8f67920f7bc962db574f105430b658c8c4791d8b1ee6e158927035b0373d3a".to_string()
        );
    }
}