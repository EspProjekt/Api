pub mod conn;
pub mod types;



use types::RedisPool;
#[derive(Debug)]
pub struct Redis {
	pub pool: RedisPool,
}
