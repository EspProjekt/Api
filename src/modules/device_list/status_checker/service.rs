use tokio::time::sleep;
use serde_json::from_str;
use tokio::time::Duration;
use reqwest::Method;
use futures::stream::StreamExt;
use futures::stream::{self};
use crate::data::device::Status;
use crate::redis::Redis;
use crate::{data::device_list::DeviceList, state::AppState, utils::Utils};
use crate::utils::constants::STATUS_CHECKER_INTERVAL_DEALY;
use crate::utils::http_request::ReqResult;


pub struct DevicesStatusChecker{
    redis: Redis,
    ips: Vec<String>,
}


impl DevicesStatusChecker {
    fn new(redis: Redis) -> Self { Self { redis, ips: Vec::new() } }


    pub async fn run(app_state: AppState) {
        let mut status_checker = Self::new(app_state.redis.clone());
        tokio::spawn(async move { status_checker.status_loop().await; });
    }


    async fn status_loop(&mut self){
        loop {
            sleep(Duration::from_secs(STATUS_CHECKER_INTERVAL_DEALY)).await;
            self.get_ip_list();
            self.update_devices().await;
        }
    }


    fn get_ip_list(&mut self) {
        self.ips = DeviceList::list_ips(&self.redis).unwrap();
    }

    
    async fn update_devices(&self) {
        if self.ips.is_empty() { return; }
        for result in self.send_requests().await {
            self.handle_result(result).await;
        }
    }
    

    async fn send_requests(&self) -> Vec<ReqResult> {
        let stream = stream::iter(self.ips.to_vec()).map(|ip| {
            async move{ Utils::send_request(ip, Method::GET, "status").await }
        });

        stream.buffer_unordered(self.ips.len()).collect::<Vec<ReqResult>>().await
    }


    async fn handle_result(&self, result: ReqResult){
        match result {
            Ok((resp, ip)) => {
                let data = resp.text().await.unwrap();
                let device_status = from_str::<Status>(&data).unwrap();
                
                println!("Device status: {:#?} for ip: {}", device_status, ip);
                DeviceList::update_device(device_status, ip, &self.redis);
            },

            Err((err, ip)) => println!("Error: {:?} for ip: {}", err, ip),
        }
    }
}