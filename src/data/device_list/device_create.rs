use super::*;

impl DeviceList{
    pub fn add_device(device: Device, redis: &Redis) -> DeviceListResult {
        let mut device_list = Self::get_device_list(redis)?;
        
        if device_list.devices.iter().any(|d| d.ip == device.ip){ return Err(Error::new(409)); }
        device_list.devices.push(device);
        
        Self::set_device_list(redis, device_list)
    }
}