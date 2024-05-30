use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

struct Device {
	ip: String,
	uptime: Option<String>,
	status: Option<String>,
}

impl Device {
    fn new(ip: &str) -> Self {
        Device {
            ip: ip.to_string,
            uptime: None,
            status: None,
        }
    }
}
