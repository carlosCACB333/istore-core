use crate::models::messages::Message;
use crate::models::users::User;
use crate::schema::chats;
use crate::utils::db::Conn;
use diesel::{pg::Pg, prelude::*};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = chats)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(Pg))]
pub struct Chat {
    pub id: String,
    pub model: String,
    pub api_key: String,
    pub user_id: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = chats)]
#[diesel(check_for_backend(Pg))]
pub struct NewChat {
    pub id: String,
    pub model: String,
    pub api_key: String,
    pub user_id: Option<i32>,
}

impl NewChat {
    pub fn new(id: String, model: String, api_key: String, user_id: Option<i32>) -> Self {
        NewChat {
            id,
            model,
            api_key,
            user_id,
        }
    }

    pub fn insert_or_update(&self, conn: &mut Conn) -> Result<Chat, diesel::result::Error> {
        diesel::insert_into(chats::table)
            .values(self)
            .on_conflict(chats::id)
            .do_update()
            .set(self)
            .get_result(conn)
    }
}

impl Chat {
    pub fn find_by_id(
        id: &str,
        conn: &mut Conn,
    ) -> Result<ChatWithMessages, diesel::result::Error> {
        let chat = chats::table.find(id).get_result::<Chat>(conn)?;
        let messages = Message::belonging_to(&chat).load::<Message>(conn)?;

        log::info!("Chat: {:?}", chat);
        log::info!("Messages: {:?}", messages);

        Ok(ChatWithMessages { chat, messages })
    }
}

#[derive(Serialize)]
pub struct ChatWithMessages {
    #[serde(flatten)]
    pub chat: Chat,
    pub messages: Vec<Message>,
}
