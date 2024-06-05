use actix_web::{web::Path, HttpResponse};
use reqwest::Method;
use serde_json::from_str;
use uuid::Uuid;
use crate::{data::{device::Status, device_list::DeviceList}, modules::device::DeviceController, state::AppState, utils::Utils};


impl DeviceController{
    pub async fn switch_light(device_id: Path<Uuid>, app_state: AppState) -> HttpResponse{
        let state = &app_state.lock().await;
        let device_ip = match DeviceList::get_device_ip(device_id.into_inner(), &state.redis) {
            Ok(ip) => ip,
            Err(e) => return e.into_response(),
        };
        
        match Utils::send_request(device_ip, Method::POST, "light").await {
            Ok((resp, ip)) => {
                let data = resp.text().await.unwrap();
                let device_status = from_str::<Status>(&data).unwrap();
                
                DeviceList::update_device(device_status, ip, &state.redis);
                HttpResponse::Ok().json(DeviceList::list_devices(&state.redis).unwrap())
            },
            Err((e, _ip)) => e.into_response(),
        }
    }
}