use std::sync::Arc;
use tokio::sync::Mutex;
use actix_web::web::Data;
use crate::ws::manager::WebSocketManager;

use super::redis::Redis;


pub type AppStateRef = Arc<Mutex<State>>;
pub type AppState = Data<AppStateRef>;
pub struct State {
	pub redis: Redis,
	pub ws_manager: WebSocketManager,
}

impl State {
	pub fn new() -> Arc<Mutex<Self>>  {
		let redis = Redis::connect();
		Arc::new(Mutex::new(Self { 
			ws_manager: WebSocketManager::new(&redis),
			redis,
		}))
	}
}




