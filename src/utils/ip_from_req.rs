use crate::errors::err::Error;
use super::Utils;
use std::net::IpAddr;
use actix_web::HttpRequest;


impl Utils {
    pub fn get_ip(req: HttpRequest) -> Result<IpAddr, Error> {
        if req.peer_addr().is_none() { return Err(Error::new(403)); }
        Ok(req.peer_addr().unwrap().ip())
    }
}