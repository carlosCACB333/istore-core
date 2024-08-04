use crate::{
    make_response,
    models::{
        chats::{Chat, ChatWithMessages, NewChat},
        messages::NewMessage,
    },
    utils::{db::Pool, tools::Status},
};
use actix_web::{get, post, web, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct MessageReq {
    pub id: String,
    pub role: String,
    pub content: serde_json::Value,
}

#[derive(Deserialize, Debug)]
pub struct ChatReq {
    pub id: String,
    pub model: String,
    pub api_key: String,
    pub messages: Vec<MessageReq>,
}

#[post("")]
pub async fn create_chat(mut pool: Pool, body: web::Json<ChatReq>) -> impl Responder {
    log::info!("Requesting estract text {:?}", body);
    let chat = NewChat::new(
        body.id.clone(),
        body.model.clone(),
        body.api_key.clone(),
        None,
    );
    let chat = match chat.insert_or_update(&mut pool.conn) {
        Ok(chat) => chat,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    let messages = body
        .messages
        .iter()
        .map(|m| {
            NewMessage::new(
                m.id.clone(),
                m.role.clone(),
                m.content.clone(),
                chat.id.clone(),
            )
        })
        .collect::<Vec<_>>();

    let messages = match NewMessage::create(&chat.id, messages, &mut pool.conn) {
        Ok(messages) => messages,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(json!(ChatWithMessages { chat, messages }))
}

#[get("/{id}")]
pub async fn get_chat_by_id(mut pool: Pool, id: web::Path<String>) -> impl Responder {
    let chat = match Chat::find_by_id(&id, &mut pool.conn) {
        Ok(chat) => chat,
        Err(e) => {
            return make_response!(Status::FAILED, e.to_string().as_str());
        }
    };

    make_response!(json!(chat))
}
