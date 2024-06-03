use std::net::IpAddr;
use std::time::Duration;
use reqwest::{Client, Response, Method};
use super::Utils;
use super::constants::REQUEST_TIMEOUT;
use crate::errors::err::Error;


pub type ReqResult = Result<(Response, IpAddr), (Error, IpAddr)>;


impl Utils {
    pub async fn send_request(ip: String, method: Method, path: &str) -> ReqResult{
        let uri = format!("http://{}/{}", ip, path);
        let client = Client::new();
        let ip = ip.parse().unwrap();

        match client.request(method, &uri)
        .timeout(Duration::from_secs(REQUEST_TIMEOUT))
        .send().await {
            Ok(json) => Ok((json, ip)),
            Err(_) => Err((Error::new(500), ip)),
        }
    }
}
