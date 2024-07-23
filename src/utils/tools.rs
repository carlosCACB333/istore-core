use actix_web::HttpResponse;
use serde::Serialize;

#[derive(Serialize, Copy, Clone)]
pub enum Status {
    SUCCESS,
    FAILED,
}

#[derive(Serialize)]
pub struct Response<'a, T: Serialize> {
    status: Status,
    message: &'a str,
    data: Option<T>,
}

pub fn make_res<T: Serialize>(status: Status, message: &str, data: T) -> HttpResponse {
    let response = Response {
        status,
        message,
        data: Some(data),
    };

    match status {
        Status::SUCCESS => HttpResponse::Ok().json(response),
        Status::FAILED => HttpResponse::BadRequest().json(response),
    }
}
