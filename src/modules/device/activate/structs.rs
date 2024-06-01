use actix_web::web;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct DeviceActivatePayload {
    pub uptime: u64,
    pub is_light_on: bool,
    pub name: String,
}


pub type Payload = web::Json<DeviceActivatePayload>;