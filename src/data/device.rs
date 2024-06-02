use std::net::IpAddr;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::modules::device::activate::structs::DeviceActivatePayload;


pub type Status = DeviceStatusResponse;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub ip: String,
    pub uptime: u64,
    pub is_light_on: bool,
    pub status: bool,
    pub last_update: u64,
    pub name: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PublicDevice {
    pub id: Uuid,
    pub uptime: u64,
    pub is_light_on: bool,
    pub status: bool,
    pub last_update: u64,
    pub name: String,
}


#[derive(Deserialize, Serialize, Debug)]
pub struct DeviceStatusResponse {
    pub is_light_on: bool,
    pub uptime: u64,
}


pub struct DeviceCreateData {
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

    pub fn update_status(&mut self, new_status: Status) {
        self.is_light_on = new_status.is_light_on;
        self.uptime = new_status.uptime;
        self.last_update = chrono::Utc::now().timestamp() as u64;
    }
}


impl PublicDevice{
    pub fn from(device: Device) -> Self {
        Self {
            id: device.id,
            uptime: device.uptime,
            is_light_on: device.is_light_on,
            status: device.status,
            last_update: device.last_update,
            name: device.name,
        }
    }
}