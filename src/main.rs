use actix_cors::Cors;
use actix_web::{web::{self, scope}, App, HttpResponse, HttpServer};
use data::device_list::DeviceList;
use dotenv::dotenv;
use router::Router;
use state::State;


pub mod modules;
pub mod redis;
pub mod router;
pub mod state;
pub mod data;
pub mod utils;
pub mod errors;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let app_data = web::Data::new(State::new());
    DeviceList::new(&app_data.redis).unwrap();
    
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
            .service(scope("/device").configure(Router::device))
            .service(scope("/device-list").configure(Router::device_list))
    })
    .bind("0.0.0.0:5000")?;

    for addr in server.addrs() {println!("Server running on http://{}", addr);}

    server.run().await
}

async fn health_check() -> HttpResponse {
	HttpResponse::Ok().into()
}
