use super::*;

impl DeviceList{
    pub fn add_device(device: Device, redis: &Redis) -> DeviceListResult {
        let mut device_list = Self::get_device_list(redis)?;
        
        if device_list.devices.iter().any(|d| d.ip == device.ip){ 
            let mut found_device = device_list.devices.clone().into_iter().find(|d| d.ip == device.ip).unwrap();
            
            if found_device.status { return Err(Error::new(409)); }

            found_device.status = true;
            Self::remove_device_by_ip(found_device.ip.parse().unwrap(), redis)?;            
            println!("Device {} has been reconnected", found_device.ip);
            
        }

        device_list.devices.push(device);
        Self::set_device_list(redis, device_list)
    }
}