use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::modules::device::activate::structs::DeviceActivatePayload;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device{
    pub id: Uuid,
    pub ip: String,
    pub uptime: u64,
    pub is_light_on: bool,
    pub status: bool,
    pub last_update: u64,
    pub name: String,
}


pub struct DeviceCreateData{
    pub ip: String,
    pub uptime: u64,
    pub name: String,
    pub is_light_on: bool,
}


impl DeviceCreateData {
    pub fn from(ip: IpAddr, payload: DeviceActivatePayload) -> Self {
        Self {
            ip: ip.to_string(),
            uptime: payload.uptime,
            name: payload.name,
            is_light_on: payload.is_light_on,
        }
    }
}


impl Device {
    pub fn from(data: DeviceCreateData) -> Self {
        Self {
            id: Uuid::new_v4(),
            ip: data.ip,
            name: data.name,
            uptime: data.uptime,
            is_light_on: data.is_light_on,
            status: true,
            last_update: chrono::Utc::now().timestamp() as u64,
        }
    }
}