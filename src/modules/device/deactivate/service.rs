use actix_web::{web::Path, HttpRequest, HttpResponse};
use uuid::Uuid;
use crate::data::device_list::DeviceList;
use crate::modules::device::DeviceController;
use crate::state::AppState;
use crate::utils::Utils;


impl DeviceController{
    pub async fn deactivate_by_ip(req: HttpRequest, app_state: AppState) -> HttpResponse {
        let state = &app_state.lock().await;
        let device_ip = match Utils::get_ip(req){
            Ok(ip) => ip,
            Err(e) => return e.into_response()
        };

        match DeviceList::remove_device_by_ip(device_ip, &state.redis) {
            Ok(_) => {
                state.ws_manager.send_device_list();
                HttpResponse::NoContent().into()
            },
            Err(e) => e.into_response(),
        }
    }


    pub async fn deactivate_by_id(id: Path<Uuid>, app_state: AppState) -> HttpResponse{
        let device_id = id.into_inner();
        let state = &app_state.lock().await;

        match DeviceList::remove_device_by_id(device_id, &state.redis) {
            Ok(_) => HttpResponse::NoContent().into(),
            Err(e) => e.into_response(),
        }
    }
}