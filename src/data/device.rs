use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone)]
pub struct Device{
    pub id: Uuid,
    pub ip: String,
    pub uptime: u64,
    pub is_light_on: bool,
    pub status: bool,
    pub last_update: u64,
}