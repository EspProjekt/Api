use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest;
use tokio::time;
use actix_web::{web, App, HttpResponse, HttpServer, Responser};
use actix_web::web::Data;

#[derive(Serialize, Deserialize, Clone)]    
struct Device {
    ip: String,
    uptime: Option<String>,
    status: Option<String>,
}

impl Device {
    fn new(ip: &str) -> Self {
        Device {
            ip : ip.to_string(),
            uptime: None,
            status: None,
        }
    }
}

async fn update_device_status(device: &mut Device, response_body: serde_json::Value) {
    if let Some(uptime) = response_body.get("uptime") {
        device.uptime = Some(uptime.to_string());
    }
    if let Some(status) = response_body.get("status") {
        device.status = Some(status.to_string());
    }
}

async fn update_status(devices: Arc<Mutex<Vec<Device>>>) {
    loop {
        {
            let devices = devices.lock().unwrap();
            for device in devices.iter() {
                if let Ok(response) = reqwest::get(&format!("http://{}/", device.ip)).await {
                    if let Ok(response_body) = response.json::<serde_json::Value>().await {
                        update_device_status(device, response_body).await;
                    }
                }
            }
        }
        time::sleep(Duration::from_secs(10)).await;
    }
}

async fn turn_on(devices: Data<Arc<Mutex<Vec<Device>>>>, web::Path((id, cmd)): web::Path<(usize, String)>) -> impl Responder {
    if cmd != "turn_on" && cmd != "turn_off" {
        return HttpResponse::UnprocessableEntity().body("Unknown method");
    }

    let mut devices = devices.lock().unwrap();

    if let Some(device) = devices.get_mut(id) {
        if let Ok(response) = reqwest::get(&format!("http://{}/{}", device.ip, cmd)).await() {
            if let Ok(response_body) = response_body::<serde_json::Value>().await {
                update_device_status(device, response_body).await;
            }
        }
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().body("Device not found")
    }
}

async fn status(devices: Data<Arc<Mutex<Vec<Device>>>>) -> impl Responder {
    let devices = devices.lock().unwrap();
    HttpResponse::Ok().json(devices.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let devices = Arc::new(Mutex::new(vec![Device::new("ip")]));
    let devices_data = Data::new(devices.clone());

    tokio::spawn(update_status(devices.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(devices_data.clone())
            .route("/status", web::get().to(status))
            .route("/{id}/{cmd}", web::post().to(turn_on))
    })
    .bind("0.0.0.0:5000")?
    .run()
    .await
}
