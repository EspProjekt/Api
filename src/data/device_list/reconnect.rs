use super::*;


impl DeviceList{
    pub fn update_attempts(device_ip: IpAddr, redis: &Redis) {
        let mut device = match Self::find_device_by_ip(device_ip, redis){
            Some(d) => d,
            None => return,
        };

        device.handle_attempts();
        Self::remove_device_by_ip(device_ip, redis).unwrap();
        Self::add_device(device, redis).unwrap();
    }


    pub fn try_to_reconnect(device_id: Uuid, redis: &Redis) {
        let mut device = match Self::find_device_by_id(device_id, redis){
            Some(d) => d,
            None => return,
        };
        
        device.handle_reconnect();
        Self::remove_device_by_id(device_id, redis).unwrap();
        Self::add_device(device, redis).unwrap();
    }
}