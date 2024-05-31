use actix_web::web;
use super::redis::Redis;


pub type AppState = web::Data<State>;
pub struct State {
	pub redis: Redis,
}

impl State {
	pub fn new() -> Self {
		Self { redis: Redis::connect() }
	}
}
