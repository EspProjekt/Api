use actix_web::App;
use tokio::time::sleep;
use serde_json::from_str;
use tokio::time::Duration;
use reqwest::{Method, Response};
use futures::stream::StreamExt;
use futures::stream::{self};
use crate::data::device::Status;
use crate::redis::Redis;
use crate::{data::device_list::DeviceList, state::AppState, utils::Utils};
use crate::errors::err::Error;


pub struct DevicesStatusesInterval;


impl DevicesStatusesInterval {
    pub async fn new(app_state: AppState) {
        tokio::spawn(async move {
            Self::update_statuses(&app_state.redis).await;
        });
    }


    async fn update_statuses(redis: &Redis){
        loop {
            let ip_list = DeviceList::list_ips(redis);
            let results = Self::send_requests(ip_list).await;
            
            for result in results {
                match result {
                    Ok((resp, ip)) => {
                        let data = resp.text().await.unwrap();
                        let device_status = from_str::<Status>(&data).unwrap();
                        println!("Device status: {:#?} for ip: {}", device_status, ip);
                        DeviceList::update_device_status(device_status, ip, redis);
                    },
    
                    Err((err, ip)) => {
                        println!("Error: {:?} for ip: {}", err, ip);
                    },
                }
            }
            
            sleep(Duration::from_secs(5)).await;
        }
    }
    
    
    pub async fn send_requests(ips: Vec<String>) -> Vec<Result<(Response, String), (Error, String)>>{
        let ips_count = ips.len();
        
        let stream = stream::iter(ips).map(|ip| {
            async move{ Utils::send_request(ip, Method::GET, "status").await }
        });
        
        stream.buffer_unordered(ips_count).collect::<Vec<Result<(Response, String), (Error, String)>>>().await
    }
}

