use actix_web::web::{delete, post, ServiceConfig};
use crate::modules::device::DeviceController;
use super::Router;


impl Router {
	pub fn device(router_cfg: &mut ServiceConfig) {
		router_cfg
        .route("/activate", post().to(DeviceController::activate))
        .route("/deactivate/ip", delete().to(DeviceController::deactivate_by_ip))
        .route("/deactivate/{id}", delete().to(DeviceController::deactivate_by_id))
        .route("/light/{id}", post().to(DeviceController::switch_light));
	}
}
