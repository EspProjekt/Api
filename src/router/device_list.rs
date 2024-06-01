use actix_web::web::{get,  ServiceConfig};
use crate::modules::device_list::DeviceListController;
use super::Router;


impl Router {
	pub fn device_list(router_cfg: &mut ServiceConfig) {
		router_cfg
        .route("", get().to(DeviceListController::get_devices));
	}
}
