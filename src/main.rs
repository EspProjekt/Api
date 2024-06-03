use actix_cors::Cors;
use actix_web::{web::{self, scope}, App, HttpRequest, HttpResponse, HttpServer};
use data::device_list::DeviceList;
use dotenv::dotenv;
use modules::device_list::status_checker::service::DevicesStatusChecker;
use router::Router;
use serde::Deserialize;
use state::State;
use env_logger::Env;
use log::{info, warn, error};
use utils::Utils;


pub mod modules;
pub mod redis;
pub mod router;
pub mod state;
pub mod data;
pub mod utils;
pub mod errors;

#[derive(Debug, Deserialize)]
struct testPayload {
    name: String,
}
async fn index(req: HttpRequest, payload: web::Json<testPayload>) -> HttpResponse {
    println!("{:#?}", payload);
    let ip = Utils::get_ip(req).unwrap();
    println!("IP: {}", ip);
    HttpResponse::Ok().body("Hello world!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
	let app_data = web::Data::new(State::new());

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    DeviceList::new(&app_data.redis).unwrap();
    DevicesStatusChecker::run(app_data.clone()).await;
    
	let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(app_data.clone())
            .route("/health", web::get().to(health_check))
            .route("/index", web::post().to(index))
            .service(scope("/device").configure(Router::device))
            .service(scope("/device-list").configure(Router::device_list))
    })
    .bind("0.0.0.0:5000")?;

    for addr in server.addrs() {info!("Server running on http://{}", addr);}

    server.run().await
}

async fn health_check() -> HttpResponse {
	HttpResponse::Ok().into()
}
