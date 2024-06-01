use r2d2_redis::redis::RedisError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{env::var, net::IpAddr};
use crate::{errors::err::Error, redis::Redis};
use super::device::Device;
use crate::errors::messages::*;


#[derive(Serialize, Debug, Deserialize)]
pub struct DeviceList{
    pub id: Uuid,
    pub devices: Vec<Device>,
}


impl DeviceList{
    pub fn new(redis: &Redis) -> Result<Self, Error> {
        let list = Self{
            id: Uuid::new_v4(),
            devices: Vec::new(),
        };

        match Self::get_from_redis(&redis){
            Ok(list) => {
                println!("{:#?} Device list already exists", list);
                return Ok(list)
            },
            Err(_) => println!("Creating new device list"),
        }

        match list.set_to_redis(&redis){
            Ok(_) => Ok(list),
            Err(_) => Err(Error::new(505)),
        }
    }


    pub fn add_device(device: Device, redis: &Redis) -> Result<(), Error> {
        let mut device_list = match Self::get_from_redis(redis){
            Ok(s) => s,
            Err(_) => return Err(Error::new(404)),
        };
        
        if device_list.devices.iter().any(|d| d.ip == device.ip){ return Err(Error::new(409)); }
        device_list.devices.push(device);
        
        match device_list.set_to_redis(redis){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(500)),
        }
    }


    pub fn remove_device(device_ip: IpAddr, redis: &Redis) -> Result<(), Error> {
        let mut device_list = match Self::get_from_redis(redis){
            Ok(d) => d,
            Err(_) => return Err(Error::new(404)), 
        };
        
        device_list.devices.retain(|d| d.ip != device_ip.to_string());
        match device_list.set_to_redis(redis){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(500)),
        }
    }


    fn set_to_redis(&self, redis: &Redis) -> Result<String, RedisError>{
        redis.save(self, Self::generate_key())
    }

    
    fn get_from_redis(redis: &Redis) -> Result<Self, String>{
        redis.get(Self::generate_key())
    }


    fn generate_key() -> String{
        let key = var("DEVICE_LIST_KEY").expect(ENV_VAR_NOT_FOUND);
        format!("device_list:{}", key)
    }
}