use actix_web::{web::Path, HttpResponse};
use uuid::Uuid;
use crate::{data::device_list::DeviceList, modules::device::DeviceController, state::AppState};


impl DeviceController{
    pub async fn reconnect_device(device_id: Path<Uuid>, app_state: AppState) -> HttpResponse {
        DeviceList::try_to_reconnect(device_id.into_inner(), &app_state.redis);
        HttpResponse::Ok().into()
    }
}