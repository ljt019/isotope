use super::Message;
use crate::database::{pool::DbPool, Chat};

const SYSTEM_PROMPT: &str = "You are a helpful coding assistant. Always strive to provide complete answers without abrupt endings.";

pub struct ChatManager {
    current_chat: Chat,
    database: DbPool,
    system_prompt: String,
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
                create_new_chat(&pool, new_chat_name)?
            }
        };

        Ok(Self {
            current_chat,
            database: pool,
            system_prompt: SYSTEM_PROMPT.to_string(),
        })
    }

    pub fn get_current_chat(&self) -> &Chat {
        &self.current_chat
    }

    pub fn get_system_prompt(&self) -> &str {
        &self.system_prompt
    }

    pub fn change_system_prompt(&mut self, new_prompt: String) {
        self.system_prompt = new_prompt;
    }

    /// Will be used when the ui is implemented to switch between chats, will be used to start a new chat manually by the user
    pub fn new_chat(&mut self) -> rusqlite::Result<()> {
        let new_chat_name = format!("Chat {}", rand::random::<u32>());
        self.current_chat = create_new_chat(&self.database, new_chat_name)?;
        Ok(())
    }

    pub fn get_current_chat_id(&self) -> i64 {
        self.current_chat.id
    }

    /// Will allow user to switch between the chat they want to view/interact with
    pub fn switch_chat(&mut self, chat_id: i64) -> rusqlite::Result<()> {
        self.current_chat = crate::database::get_chat(&self.database, chat_id)?;
        Ok(())
    }

    /// Used to get all chats that have been created to display in the ui as a list for the user to select
    pub fn get_all_chats(&self) -> rusqlite::Result<Vec<Chat>> {
        crate::database::get_all_chats(&self.database)
    }

    pub fn handle_prompt(&mut self, prompt: Message) -> rusqlite::Result<Vec<Message>> {
        crate::database::insert_message_into_chat(&self.database, self.current_chat.id, &prompt)?;
        self.current_chat.messages.push(prompt);
        let system_prompt = Message {
            role: "system".to_string(),
            content: self.system_prompt.to_string(),
        };
        self.current_chat.messages.insert(0, system_prompt);
        Ok(self.current_chat.messages.clone())
    }

    pub fn handle_response(&mut self, response: Message) -> rusqlite::Result<()> {
        crate::database::insert_message_into_chat(&self.database, self.current_chat.id, &response)?;
        self.current_chat.messages.push(response);
        Ok(())
    }
}

fn create_new_chat(pool: &DbPool, name: String) -> rusqlite::Result<Chat> {
    let new_chat_id = crate::database::insert_chat(pool, name)?;
    crate::database::get_chat(pool, new_chat_id)
}
