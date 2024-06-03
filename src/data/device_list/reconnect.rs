use super::*;


impl DeviceList{
    fn perform_operation<F>(device_ip: IpAddr, redis: &Redis, func: F)
    where
        F: Fn(&mut Device),
    {
        let mut device = match Self::find_device_by_ip(device_ip, redis) {
            Some(d) => d,
            None => return,
        };

        func(&mut device);
        Self::remove_device_by_ip(device_ip, redis).unwrap();
        Self::add_device(device, redis).unwrap();
    }


    pub fn update_attempts(device_ip: IpAddr, redis: &Redis) {
        Self::perform_operation(device_ip, redis, |d| d.handle_attempts());
    }


    pub fn try_to_reconnect(device_id: Uuid, redis: &Redis) {
        let device_ip = Self::get_device_ip(device_id, redis).unwrap().parse().unwrap();
        Self::perform_operation(device_ip, redis, |d| d.handle_reconnect());
    }
}