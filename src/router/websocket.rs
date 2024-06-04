use super::Router;
use crate::{state::AppState, ws::session::Session};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix_web::Error as ActixError;


impl Router{
    pub async fn websocket(req: HttpRequest, stream: web::Payload, app_state: AppState) -> Result<HttpResponse, ActixError> {
        let connection = Session::new(app_state);

        ws::start(
            connection,
            &req,
            stream
        )
    }
}