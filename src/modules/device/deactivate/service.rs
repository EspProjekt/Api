use actix_web::{HttpRequest, HttpResponse};
use crate::data::device_list::DeviceList;
use crate::modules::device::DeviceController;
use crate::state::AppState;
use crate::utils::Utils;


impl DeviceController{
    pub async fn deactivate(req: HttpRequest, app_state: AppState) -> HttpResponse {
        let device_ip = match Utils::get_ip(req){
            Ok(ip) => ip,
            Err(e) => return e.into_response()
        };
        
        match DeviceList::remove_device(device_ip, &app_state.redis) {
            Ok(_) => HttpResponse::NoContent().into(),
            Err(e) => e.into_response(),
        }
    }
}