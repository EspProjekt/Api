pub mod conn;
pub mod types;

pub struct Redis {
	pub pool: types::RedisPool,
}
