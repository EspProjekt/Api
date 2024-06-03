use super::*;


impl DeviceList{
    pub fn generate_key() -> String{
        let key = var("DEVICE_LIST_KEY").expect(ENV_VAR_NOT_FOUND);
        format!("device_list:{}", key)
    }


    pub fn get_device_list(redis: &Redis) -> Result<Self, Error> {
        match Self::get_from_redis(redis){
            Ok(d) => Ok(d),
            Err(_) => Err(Error::new(404)),
        }
    }
    
    
    pub fn get_from_redis(redis: &Redis) -> Result<Self, String>{
        redis.get(Self::generate_key())
    }


    pub fn set_device_list(redis: &Redis, device_list: Self) -> DeviceListResult {
        match device_list.set_to_redis(redis){
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(500)),
        }
    }


    pub fn set_to_redis(&self, redis: &Redis) -> Result<String, RedisError>{
        redis.save(self, Self::generate_key())
    }
}