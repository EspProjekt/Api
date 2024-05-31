use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, Responder, HttpServer};
use dotenv::dotenv;
use state::State;

pub mod modules;
pub mod redis;
pub mod router;
pub mod state;
pub mod data;


async fn index(req: HttpRequest) -> impl Responder {
    if let Some(peer_addr) = req.peer_addr() {
        format!("Client IP: {}", peer_addr.ip())
    } else {
        "Client IP: Not found".to_string()
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	dotenv().ok();
	let app_data = web::Data::new(State::new());

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
            .route("/", web::get().to(index))
    })
    .bind("localhost:5005")?;

    for addr in server.addrs() {
        println!("Server running on http://{}", addr);
    }

    server.run().await
}

async fn health_check() -> HttpResponse {
	HttpResponse::Ok().into()
}
