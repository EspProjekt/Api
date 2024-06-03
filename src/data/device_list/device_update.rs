use super::*;

impl DeviceList{
    pub fn update_device(new_status: Status, device_ip: IpAddr, redis: &Redis) {
        let mut device = match Self::find_device_by_ip(device_ip, redis){
            Some(d) => d,
            None => return,
        };

        device.update_status(new_status);
        Self::remove_device_by_ip(device_ip, redis).unwrap();
        Self::add_device(device, redis).unwrap();
    }
}