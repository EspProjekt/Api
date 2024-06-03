use tokio::time::sleep;
use serde_json::from_str;
use tokio::time::Duration;
use reqwest::Method;
use futures::stream::StreamExt;
use futures::stream::{self};
use log::{error, info, warn};
use std::time::Instant;
use crate::data::device::{Device, Status};
use crate::redis::Redis;
use crate::{data::device_list::DeviceList, state::AppState, utils::Utils};
use crate::utils::constants::STATUS_CHECKER_INTERVAL_DEALY;
use crate::utils::http_request::ReqResult;


pub struct DevicesStatusChecker{
    redis: Redis,
    devices: Vec<DeviceToCheck>,
}


#[derive(Clone)]
pub struct DeviceToCheck{
    ip: String,
    status: bool,
    name: String,
    retry: bool,
}


impl DeviceToCheck{
    pub fn from(device: Device) -> Self {
        Self {
            ip: device.ip,
            status: device.status,
            name: device.name,
            retry: device.retry,
        }
    }
}


impl DevicesStatusChecker {
    fn new(redis: Redis) -> Self { Self { redis, devices: Vec::new() } }


    pub async fn run(app_state: AppState) {
        let mut status_checker = Self::new(app_state.redis.clone());
        tokio::spawn(async move { status_checker.status_loop().await; });
    }


    async fn status_loop(&mut self){
        loop {
            sleep(Duration::from_secs(STATUS_CHECKER_INTERVAL_DEALY)).await;
            let start = Instant::now();
            
            self.get_device_list();
            self.update_devices().await;

            info!("Status check took: {:?} for {} devices!", start.elapsed(), self.devices.len());
        }
    }


    fn get_device_list(&mut self) {
        let devices = DeviceList::get_devices_to_update(&self.redis).unwrap();
        self.devices = devices.into_iter().filter(|d| {
            if d.status || d.retry { return true }

            warn!("Device {} with ip: {} is dissabled, Skipping!", d.name, d.ip);
            false
        }).collect::<Vec<DeviceToCheck>>();
    }

    
    async fn update_devices(&self) {
        if self.devices.is_empty() { return; }
        for result in self.send_requests().await {
            self.handle_result(result).await;
        }
    }
    

    async fn send_requests(&self) -> Vec<ReqResult> {
        let stream = stream::iter(self.devices.clone()).map(|device| {
            async move { 
                Utils::send_request(device.ip, Method::GET, "status").await }
        });

        stream.buffer_unordered(self.devices.len()).collect::<Vec<ReqResult>>().await
    }


    async fn handle_result(&self, result: ReqResult){
        match result {
            Ok((resp, ip)) => {
                let data = resp.text().await.unwrap();
                let device_status = from_str::<Status>(&data).unwrap();
                
                info!("Device status: {:#?} for ip: {}", device_status, ip);
                DeviceList::update_device(device_status, ip, &self.redis);
            },

            Err((err, ip)) => {
                error!("Error: {:?} for ip: {}", err, ip);
                DeviceList::update_attempts(ip, &self.redis)
            },
        }
    }
}