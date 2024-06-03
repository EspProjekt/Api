use std::net::IpAddr;
use std::str::FromStr;
use actix_web::{HttpRequest, HttpResponse};
use log::info;
use crate::data::device::{Device, DeviceCreateData};
use crate::data::device_list::DeviceList;
use crate::modules::device::DeviceController;
use crate::state::AppState;
use crate::utils::Utils;
use super::structs::Payload;


impl DeviceController{
    pub async fn activate(req: HttpRequest, payload: Payload, app_state: AppState) -> HttpResponse {
        /* let device_ip = match Utils::get_ip(req){
            Ok(ip) => ip,
            Err(e) => return e.into_response()
        }; */
        let device_ip = "192.168.1.105";
        let addr = IpAddr::from_str(&device_ip).unwrap();

        let payload = payload.into_inner();
        let device_data = DeviceCreateData::from(addr, payload);
        let device = Device::from(device_data);

        match DeviceList::add_device(device, &app_state.redis){
            Ok(_) => {
                info!("Device added: {:?}", device_ip);
                HttpResponse::Ok().json(device_ip)
            },
            Err(e) => e.into_response()
        }
    }
}