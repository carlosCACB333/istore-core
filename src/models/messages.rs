use crate::schema::messages;
use crate::{models::chats::Chat, utils::db::Conn};
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = messages)]
#[diesel(belongs_to(Chat))]
#[diesel(check_for_backend(Pg))]
pub struct Message {
    pub id: String,
    pub role: String,
    pub content: serde_json::Value,
    pub chat_id: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, Serialize, AsChangeset, Deserialize, Debug)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(Pg))]
pub struct NewMessage {
    pub id: String,
    pub role: String,
    pub content: serde_json::Value,
    pub chat_id: String,
}

impl NewMessage {
    pub fn new(id: String, role: String, content: serde_json::Value, chat_id: String) -> Self {
        NewMessage {
            id,
            role,
            content,
            chat_id,
        }
    }

    pub fn create(
        chat_id: &str,
        messages: Vec<NewMessage>,
        conn: &mut Conn,
    ) -> Result<Vec<Message>, diesel::result::Error> {
        // delete all
        diesel::delete(messages::table.filter(messages::chat_id.eq(chat_id))).execute(conn)?;

        // insert all
        diesel::insert_into(messages::table)
            .values(&messages)
            .load(conn)
    }
}

impl Message {}
