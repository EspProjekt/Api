use super::types::*;
use super::Redis;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;
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
}
