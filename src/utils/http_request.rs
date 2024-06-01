use reqwest::{Client, Method};
use super::Utils;
use crate::errors::err::Error;


impl Utils {
    pub async fn send_request(ip: String, method: Method, path: &str) -> Result<(), Error>{
        let uri = format!("http://{}/{}", ip, path);
        let client = Client::new();
    
        match client.request(method, &uri).send().await {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(500)),
        }
    }
}
