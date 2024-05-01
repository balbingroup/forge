use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    pub address: String,
    #[serde(rename = "restApiUrl")]
    pub rest_api_url: Option<String>,
    pub name: String,
    #[serde(rename = "lastMessage")]
    pub last_message: u64,
    #[serde(rename = "lastHandshake")]
    pub last_handshake: u64,
    #[serde(rename = "connectionType")]
    pub connection_type: Option<String>,
}

impl Peer {
    pub fn new(address: &str, rest_api_url: Option<&str>, name: &str, last_message: u64, last_handshake: u64, connection_type: Option<&str>) -> Self {
        Self {
            address: address.to_string(),
            rest_api_url: rest_api_url.map(|x| x.to_string()),
            name: name.to_string(),
            last_message,
            last_handshake,
            connection_type: connection_type.map(|x| x.to_string()),
        }
    }
}