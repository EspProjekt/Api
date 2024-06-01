use actix_web::web::{post,  ServiceConfig};
use crate::modules::device::DeviceController;

use super::Router;


impl Router {
	pub fn device(router_cfg: &mut ServiceConfig) {
		router_cfg
        .route("/activate", post().to(DeviceController::activate))
        .route("/deactivate", post().to(DeviceController::deactivate));
	}
}
