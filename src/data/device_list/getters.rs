use super::*;


impl DeviceList{
    pub fn list_devices(redis: &Redis) -> Result<Vec<PublicDevice>, Error> {
        let device_list = Self::get_device_list(&redis)?;
        Ok(device_list.devices.into_iter().map(|d| PublicDevice::from(d)).collect::<Vec<PublicDevice>>())
    }


    pub fn get_devices_to_update(redis: &Redis) -> Result<Vec<DeviceToCheck>, Error>{
        let device_list = Self::get_device_list(redis)?; 
        Ok(device_list.devices.into_iter().map(|d| DeviceToCheck::from(d)).collect())
    }
}