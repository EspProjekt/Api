use r2d2_redis::redis::RedisError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::{env::var, net::IpAddr};
use crate::{errors::err::Error, redis::Redis};
use super::device::{Device, PublicDevice, Status};
use crate::errors::messages::*;


#[derive(Debug, Serialize, Deserialize)]
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
        let mut device_list = Self::get_device_list(redis)?;
        
        if device_list.devices.iter().any(|d| d.ip == device.ip){ return Err(Error::new(409)); }
        device_list.devices.push(device);
        
        Self::set_device_list(redis, device_list)
    }


    pub fn remove_device<F>(redis: &Redis, filter: F) -> Result<(), Error>
    where
        F: Fn(&Device) -> bool,
    {
        let mut device_list = Self::get_device_list(&redis)?;
        device_list.devices.retain(|d| !filter(d));

        Self::set_device_list(redis, device_list)
    }

    
    pub fn remove_device_by_ip(device_ip: IpAddr, redis: &Redis) -> Result<(), Error> {
        Self::remove_device(redis, |d| d.ip == device_ip.to_string())
    }
    

    pub fn remove_device_by_id(device_id: Uuid, redis: &Redis) -> Result<(), Error> {
        Self::remove_device(redis, |d| d.id == device_id)
    }
    

    pub fn get_device_ip(device_id: Uuid, redis: &Redis) -> Result<String, Error> {
        let device_list = Self::get_device_list(redis)?;

        match device_list.devices.iter().find(|d| d.id == device_id){
            Some(d) => Ok(d.ip.clone()),
            None => Err(Error::new(404)),
        }
    }


    pub fn list_devices(redis: &Redis) -> Result<Vec<PublicDevice>, Error> {
        let device_list = Self::get_device_list(&redis)?;
        Ok(device_list.devices.into_iter().map(|d| PublicDevice::from(d)).collect::<Vec<PublicDevice>>())
    }


    pub fn list_ips(redis: &Redis) -> Result<Vec<String>, Error>{
        let device_list = Self::get_device_list(redis)?;
        Ok(device_list.devices.iter().map(|d| d.ip.clone()).collect::<Vec<String>>())
    }

    
    pub fn update_device(new_status: Status, device_ip: String, redis: &Redis) {
        let device_list = Self::get_device_list(redis).unwrap();

        let mut device = match device_list.devices.into_iter().find(|d| d.ip == device_ip) {
            Some(d) => d,
            None => return,
        };

        device.update_status(new_status);
        Self::remove_device_by_ip(device_ip.parse().unwrap(), redis).unwrap();
        Self::add_device(device, redis).unwrap();
    }
    

    // redis part
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


    fn get_device_list(redis: &Redis) -> Result<Self, Error> {
        match Self::get_from_redis(redis){
            Ok(d) => Ok(d),
            Err(_) => Err(Error::new(404)),
        }
    }

    fn set_device_list(redis: &Redis, d_list: Self) -> Result<(), Error> {
        match d_list.set_to_redis(redis){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(500)),
        }
    }
}