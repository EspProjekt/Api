pub mod conn;
pub mod types;



use types::RedisPool;
pub struct Redis {
	pub pool: RedisPool,
}
