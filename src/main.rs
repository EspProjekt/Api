use actix_cors::Cors;
use actix_web::{web::{self, scope}, App, HttpResponse, HttpServer};
use data::device_list::DeviceList;
use dotenv::dotenv;
use modules::device_list::status_checker::service::DevicesStatusChecker;
use router::Router;
use state::{AppState, State};
use env_logger::Env;
use log::info;
use env_logger::Builder;
use web::{get, Data};


pub mod modules;
pub mod redis;
pub mod router;
pub mod state;
pub mod data;
pub mod utils;
pub mod errors;
pub mod ws;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let app_state = State::new();
    DevicesStatusChecker::run(&app_state).await;
    
    {
        // w inny scope musi isc lock na app_data
        let state = app_state.lock().await;
        Builder::from_env(Env::default().default_filter_or("info")).init();
        DeviceList::new(&state.redis).unwrap();
    }
    
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
            .app_data(Data::new(app_state.clone()))
            .route("/health", get().to(health_check))
            .route("/session", get().to(Router::websocket))
            .service(scope("/device").configure(Router::device))
            .service(scope("/device-list").configure(Router::device_list))
    })
    .bind("0.0.0.0:5000")?;

    for addr in server.addrs() {info!("Server running on http://{}", addr);}
    server.run().await
}

async fn health_check(app_state: AppState) -> HttpResponse {
    let data = app_state.lock().await;
    data.ws_manager.send_device_list();
    HttpResponse::Ok().into()
}