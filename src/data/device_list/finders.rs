use super::*;


impl DeviceList{
    pub fn find_device_by_ip(ip: IpAddr, redis: &Redis) -> Option<Device>{
        let device_list = Self::get_device_list(redis).unwrap();
        device_list.devices.into_iter().find(|d| d.ip == ip.to_string())
    }


    pub fn find_device_by_id(id: Uuid, redis: &Redis) -> Option<Device>{
        let device_list = Self::get_device_list(redis).unwrap();
        device_list.devices.into_iter().find(|d| d.id == id)
    }

    
    pub fn get_device_ip(device_id: Uuid, redis: &Redis) -> Result<String, Error> {
        let device_list = Self::get_device_list(redis)?;

        match device_list.devices.iter().find(|d| d.id == device_id){
            Some(d) => Ok(d.ip.clone()),
            None => Err(Error::new(404)),
        }
    }
}