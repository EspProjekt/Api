use super::*;

impl DeviceList{
    pub fn remove_device<F>(redis: &Redis, filter: F) -> DeviceListResult
    where
        F: Fn(&Device) -> bool,
    {
        let mut device_list = Self::get_device_list(&redis)?;
        device_list.devices.retain(|d| !filter(d));

        Self::set_device_list(redis, device_list)
    }

    
    pub fn remove_device_by_ip(device_ip: IpAddr, redis: &Redis) -> DeviceListResult {
        Self::remove_device(redis, |d| d.ip == device_ip.to_string())
    }
    

    pub fn remove_device_by_id(device_id: Uuid, redis: &Redis) -> DeviceListResult {
        Self::remove_device(redis, |d| d.id == device_id)
    }
    
}