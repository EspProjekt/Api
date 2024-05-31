use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use state::State;

pub mod modules {}

pub mod redis;
pub mod router;
pub mod state;
pub mod data;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let app_data = web::Data::new(State::new());

	HttpServer::new(move || {
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
	})
	.bind("0.0.0.0:5000")?
	.run()
	.await
}

async fn health_check() -> HttpResponse {
	HttpResponse::Ok().into()
}
