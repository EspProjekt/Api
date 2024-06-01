use super::messages::*;
use actix_web::{http::StatusCode, HttpResponse};
use serde::{Deserialize, Serialize};


#[derive(thiserror::Error, Debug)]
#[error("...")]
pub enum Error {
	#[error("{:?}", .0)]
	Web(StatusCode),
}


#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
	message: String,
}


impl ErrorResponse {
    pub fn new(code: StatusCode) -> Self {
        Self {
            message: Self::get_message(code),
        }
    }


    fn get_message(code: StatusCode) -> String {
		match code.as_u16() {
			404 => NOT_FOUND_MSG,
			401 => UNAUTHORIZED_MSG,
			500 => INTERNAL_SERVER_ERROR_MSG,
			422 => UNPROCESSABLE_ENTITY_MSG,
            403 => FORBIDDEN_MSG,

			_ => INTERNAL_SERVER_ERROR_MSG,
		}.to_string()
	}
}


impl Error {
	pub fn new(code: u16) -> Self {
		let status = StatusCode::from_u16(code).unwrap_or_default();
		Error::Web(status)
	}


	pub fn into_response(&self) -> HttpResponse {
		HttpResponse::build(self.get_code()).json(ErrorResponse::new(self.get_code()))
	}


    fn get_code(&self) -> StatusCode {
		match *self {
			Error::Web(code) => code,
		}
	}
}