use log::{info, warn};
use r2d2_redis::redis::RedisError;
use uuid::Uuid;
use std::{env::var, net::IpAddr};
use serde::{Deserialize, Serialize};
use crate::{errors::err::Error, modules::device_list::status_checker::service::DeviceToCheck, redis::Redis};
use crate::data::device::{Device, PublicDevice, Status};
use crate::errors::messages::*;


pub type DeviceListResult = Result<(), Error>;


#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceList{
    pub id: Uuid,
    pub devices: Vec<Device>,
}


impl DeviceList {
    pub fn new(redis: &Redis) -> Result<Self, Error> {
        let list = Self{
            id: Uuid::new_v4(),
            devices: Vec::new(),
        };

        match Self::get_from_redis(&redis){
            Ok(list) => {
                info!("Device list already exists {:#?}", list);
                return Ok(list)
            },
            Err(_) => warn!("Creating new device list"),
        }

        match list.set_to_redis(&redis){
            Ok(_) => Ok(list),
            Err(_) => Err(Error::new(505)),
        }
    }
}


pub mod device_create;
pub mod device_remove;
pub mod device_update;
pub mod getters;
pub mod finders;
pub mod reconnect;
pub mod redis;