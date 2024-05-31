use r2d2_redis::redis::RedisError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env::var;
use crate::redis::Redis;
use super::device::Device;
use super::messages::*;


#[derive(Serialize, Deserialize)]
pub struct DeviceList{
    pub id: Uuid,
    pub devices: Vec<Device>,
}


impl DeviceList{
    pub fn new(redis: Redis) -> Result<Self, &'static str> {
        let list = Self{
            id: Uuid::new_v4(),
            devices: Vec::new(),
        };

        match Self::get_from_redis(&redis){
            Ok(s) => return Ok(s),
            Err(_) => {},
        }

        match list.set_to_redis(&redis){
            Ok(_) => Ok(list),
            Err(_) => Err(FAILED_TO_SAVE_D_LIST),
        }
    }


    pub fn add_device(device: Device, redis: &Redis) -> &'static str {
        let mut device_list = match Self::get_from_redis(redis){
            Ok(s) => s,
            Err(_) => return FAILED_TO_GET_D_LIST,
        };
        
        device_list.devices.push(device);
        match device_list.set_to_redis(redis){
            Ok(_) => DEVICE_ADDED,
            Err(_) => FAILED_TO_SAVE_D_LIST,
        }
    }


    pub fn remove_device(device_id: Uuid, redis: &Redis) -> &'static str {
        let mut device_list = match Self::get_from_redis(redis){
            Ok(d) => d,
            Err(_) => return FAILED_TO_GET_D_LIST, 
        };
        
        device_list.devices.retain(|d| d.id != device_id);
        match device_list.set_to_redis(redis){
            Ok(_) => DEVICE_REMOVED,
            Err(_) => FAILED_TO_SAVE_D_LIST,
        }
    }


    pub fn set_to_redis(&self, redis: &Redis) -> Result<String, RedisError>{
        redis.save(self, Self::generate_key())
    }

    
    pub fn get_from_redis(redis: &Redis) -> Result<Self, String>{
        redis.get(Self::generate_key())
    }


    fn generate_key() -> String{
        let key = var("DEVICE_LIST_KEY").expect(ENV_VAR_NOT_FOUND);
        format!("device_list:{}", key)
    }
}