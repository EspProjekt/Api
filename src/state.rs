use super::redis::Redis;

pub struct State {
	pub redis: Redis,
}

impl State {
	pub fn new() -> Self {
		Self { redis: Redis::connect() }
	}
}
