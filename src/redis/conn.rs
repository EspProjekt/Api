use super::types::*;
use super::Redis;
use r2d2::Pool;
use r2d2_redis::redis::Commands;
use r2d2_redis::redis::RedisError;
use r2d2_redis::RedisConnectionManager;
use serde::de::DeserializeOwned;
use serde_json::to_string;
use std::env;


impl Redis {
	pub fn connect() -> Self {
		let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
		let manager = RedisConnectionManager::new(redis_url).unwrap();

		Self {
			pool: Pool::builder().build(manager).expect("Failed to create pool."),
		}
	}


	pub fn get_conn(&self) -> RedisConn {
		self.pool.get().expect("Failed to get connection.")
	}


	pub fn save<T>(&self, value: T, key: String) -> Result<String, RedisError>
	where
		T: serde::Serialize,
	{
		let mut conn = self.get_conn();
		let data_str = to_string(&value).expect("Failed to serialize data.");

		match conn.set(key, data_str) {
			Ok(a) => Ok(a),
			Err(e) => Err(e),
		}
	}


	pub fn get<T>(&self, key: String) -> Result<T, String>
	where
		T: DeserializeOwned,
	{
		let mut conn = self.get_conn();
		let data_str: String = match conn.get(key) {
			Ok(d) => d,
			Err(_e) => return Err(String::from("Failed to get data.")),
		};

		match serde_json::from_str(&data_str) {
			Ok(d) => Ok(d),
			Err(_e) => Err(String::from("Failed to parse data.")),
		}
	}
}
