use actix_web::HttpResponse;
use crate::{data::device_list::DeviceList, modules::device_list::DeviceListController, state::AppState};


impl DeviceListController{
    pub async fn get_devices(app_state: AppState) -> HttpResponse {
        match DeviceList::list_devices(&app_state.redis) {
            Ok(devices) => HttpResponse::Ok().json(devices),
            Err(e) => e.into_response(),
        }
    }
}