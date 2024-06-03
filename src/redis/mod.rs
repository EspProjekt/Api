pub mod conn;
pub mod types;



use types::RedisPool;
#[derive(Debug, Clone)]
pub struct Redis {
	pub pool: RedisPool,
}
