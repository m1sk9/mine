use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusResponse {
    pub online: bool,
    pub version: Option<String>,
    pub players: PlayerlistResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerData {
    pub name: String,
    pub uuid: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerlistResponse {
    pub online: u64,
    pub max: u64,
    pub list: Option<Vec<PlayerData>>,
}
