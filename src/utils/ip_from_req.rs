use super::Utils;
use std::net::IpAddr;
use actix_web::HttpRequest;


impl Utils {
    pub fn get_ip(req: HttpRequest) -> Result<IpAddr, &'static str> {
        if req.peer_addr().is_none() { return Err("Could not get IP address"); }
        Ok(req.peer_addr().unwrap().ip())
    }
}