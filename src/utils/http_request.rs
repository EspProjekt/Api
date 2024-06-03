use reqwest::{Client, Response, Method};
use super::Utils;
use crate::errors::err::Error;


pub type ReqResult = Result<(Response, String), (Error, String)>;

impl Utils {
    pub async fn send_request(ip: String, method: Method, path: &str) -> ReqResult{
        let uri = format!("http://{}/{}", ip, path);
        let client = Client::new();
    
        match client.request(method, &uri).send().await {
            Ok(json) => Ok((json, ip)),
            Err(_) => Err((Error::new(500), ip)),
        }
    }
}
