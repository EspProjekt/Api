use r2d2_redis::redis::RedisError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env::var;
use crate::redis::Redis;
use super::device::Device;



#[derive(Serialize, Deserialize)]
pub struct DeviceList{
    pub id: Uuid,
    pub devices: Vec<Device>,
}


impl DeviceList{
    pub fn new(redis: Redis) -> Result<Self, String>{
        let list = Self{
            id: Uuid::new_v4(),
            devices: Vec::new(),
        };

        match Self::get_self(&redis){
            Ok(s) => return Ok(s),
            Err(_) => {},
        }

        match list.set_self(&redis){
            Ok(_) => Ok(list),
            Err(_) => Err(String::from("Failed to save device list")),
        }
    }


    pub fn add_device(device: Device, redis: &Redis) -> String {
        let mut device_list = match Self::get_self(redis){
            Ok(s) => s,
            Err(_) => return String::from("Failed to get device list"),
        };

        device_list.devices.push(device);
        match device_list.set_self(redis){
            Ok(_) => String::from("Device added"),
            Err(_) => String::from("Failed to save device list"),
        }

    }


    pub fn set_self(&self, redis: &Redis) -> Result<String, RedisError>{
        redis.save(self, Self::generate_key())
    }

    
    pub fn get_self(redis: &Redis) -> Result<Self, String>{
        redis.get(Self::generate_key())
    }


    fn generate_key() -> String{
        let key = var("DEVICE_LIST_KEY").unwrap();
        format!("device_list:{}", key)
    }
}