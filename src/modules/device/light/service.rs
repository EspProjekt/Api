use actix_web::{web::Path, HttpResponse};
use reqwest::Method;
use uuid::Uuid;
use crate::{data::device_list::DeviceList, modules::device::DeviceController, utils::Utils};


impl DeviceController{
    pub async fn switch_light(device_id: Path<Uuid>) -> HttpResponse{
        let device_id = device_id.into_inner();
        let device_ip = match DeviceList::get_device_ip(device_id) {
            Ok(ip) => ip,
            Err(e) => return e.into_response(),
        };
        
        match Utils::send_request(device_ip, Method::GET, "light").await {
            Ok(_) => HttpResponse::Ok().into(),
            Err(e) => e.into_response(),
        }
    }
}