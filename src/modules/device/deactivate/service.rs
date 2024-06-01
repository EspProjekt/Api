use actix_web::{HttpRequest, HttpResponse};
use crate::modules::device::DeviceController;
use crate::utils::Utils;


impl DeviceController{
    pub async fn deactivate(req: HttpRequest) -> HttpResponse {
        let device_ip = match Utils::get_ip(req){
            Ok(ip) => ip,
            Err(e) => return HttpResponse::BadRequest().json(e)
        };

        HttpResponse::Ok().json("Device deactivated")
    }
}