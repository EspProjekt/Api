use tokio::time::sleep;
use tokio::time::Duration;
use reqwest::{Method, Response};
use futures::stream::StreamExt;
use futures::stream::{self};
use crate::{data::device_list::DeviceList, modules::device_list::DeviceListController, state::AppState, utils::Utils};
use crate::errors::err::Error;


impl DeviceListController{
    pub async fn update_statuses(app_state: AppState){
        loop {
            let ip_list = DeviceList::list_ips(&app_state.redis);
            let responses = send_requests(ip_list).await;

            sleep(Duration::from_secs(5)).await;
        }
    }
}


pub async fn send_requests(ips: Vec<String>) -> Vec<Result<(Response, String), (Error, String)>>{
    let ips_count = ips.len();
    
    let stream = stream::iter(ips).map(|ip| {
        async move{ Utils::send_request(ip, Method::GET, "status").await }
    });
    
    stream.buffer_unordered(ips_count).collect::<Vec<Result<(Response, String), (Error, String)>>>().await
}