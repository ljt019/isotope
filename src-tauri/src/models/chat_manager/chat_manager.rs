use super::Message;
use crate::database::{pool::DbPool, Chat};

pub struct ChatManager {
    current_chat: Chat,
    database: DbPool,
}

impl ChatManager {
    pub fn new(pool: DbPool) -> rusqlite::Result<Self> {
        // Setup chat table if it doesn't exist
        crate::database::setup_database(&pool);

        // Try to load most recent chat from database
        let current_chat = match crate::database::get_most_recent_chat(&pool)? {
            Some(chat_id) => crate::database::get_chat(&pool, chat_id)?,
            None => {
                // Generate random chat name and create new chat
                let new_chat_name = format!("Chat {}", rand::random::<u32>());
                let new_chat_id = crate::database::insert_chat(&pool, new_chat_name)?;
                crate::database::get_chat(&pool, new_chat_id)?
            }
        };

        Ok(Self {
            current_chat,
            database: pool,
        })
    }

    pub fn get_current_chat(&self) -> &Chat {
        &self.current_chat
    }

    pub fn new_chat(&mut self) -> rusqlite::Result<()> {
        let new_chat_name = format!("Chat {}", rand::random::<u32>());
        let new_chat_id = crate::database::insert_chat(&self.database, new_chat_name)?;
        self.current_chat = crate::database::get_chat(&self.database, new_chat_id)?;
        Ok(())
    }

    pub fn switch_chat(&mut self, chat_id: i64) -> rusqlite::Result<()> {
        self.current_chat = crate::database::get_chat(&self.database, chat_id)?;
        Ok(())
    }

    pub fn get_all_chats(&self) -> rusqlite::Result<Vec<Chat>> {
        crate::database::get_all_chats(&self.database)
    }

    pub fn handle_prompt(&mut self, prompt: Message) -> rusqlite::Result<Vec<Message>> {
        crate::database::insert_message_into_chat(&self.database, self.current_chat.id, &prompt)?;
        self.current_chat.messages.push(prompt);
        Ok(self.current_chat.messages.clone())
    }

    pub fn handle_response(&mut self, response: Message) -> rusqlite::Result<()> {
        crate::database::insert_message_into_chat(&self.database, self.current_chat.id, &response)?;
        self.current_chat.messages.push(response);
        Ok(())
    }
}
