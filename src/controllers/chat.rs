use crate::services::chat::{create_chat, get_chat_by_id};
use actix_web::web;

pub fn routes() -> actix_web::Scope {
    web::scope("/chat")
        .service(create_chat)
        .service(get_chat_by_id)
}
