use actix_web::{web::Path, HttpResponse};
use reqwest::Method;
use uuid::Uuid;
use crate::{data::device_list::DeviceList, modules::device::DeviceController, state::AppState, utils::Utils};


impl DeviceController{
    pub async fn switch_light(device_id: Path<Uuid>, app_state: AppState) -> HttpResponse{
        let device_ip = match DeviceList::get_device_ip(device_id.into_inner(), &app_state.redis) {
            Ok(ip) => ip,
            Err(e) => return e.into_response(),
        };
        
        match Utils::send_request(device_ip, Method::POST, "light").await {
            Ok(_) => HttpResponse::Ok().into(),
            Err((e, _ip)) => e.into_response(),
        }
    }
}