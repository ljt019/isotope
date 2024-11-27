use crate::database::Chat;
use crate::types::Message;

pub struct ChatManager {
    current_chat: Chat,
    database: rusqlite::Connection,
}

impl ChatManager {
    pub fn new(database_connection: rusqlite::Connection) -> Self {
        // Try to load most recent chat from database
        // If no chat is found, create a new chat
        let most_recent_chat_id = crate::database::get_most_recent_chat(&database_connection)
            .expect("Failed to get most recent chat");

        let current_chat = match most_recent_chat_id {
            Some(most_recent_chat_id) => {
                // Load the most recent chat
                crate::database::get_chat(&database_connection, most_recent_chat_id)
            }
            None => {
                // Generate random chat name
                let new_chat_name = format!("Chat {}", rand::random::<u32>());

                // Create a new chat
                let new_chat_id = crate::database::insert_chat(&database_connection, new_chat_name)
                    .expect("Failed to create new chat");

                // Load the new chat
                crate::database::get_chat(&database_connection, new_chat_id)
            }
        };

        Self {
            current_chat: current_chat,
            database: database_connection,
        }
    }

    pub fn new_chat(&mut self) {
        // Generate random chat name
        let new_chat_name = format!("Chat {}", rand::random::<u32>());

        // Create a new chat in the database
        let new_chat_id = crate::database::insert_chat(&self.database, new_chat_name)
            .expect("Failed to create new chat");

        // Load the new chat
        self.current_chat = crate::database::get_chat(&self.database, new_chat_id);
    }

    pub fn switch_chat(&mut self, chat_id: i64) {
        // Load the chat from the database
        self.current_chat = crate::database::get_chat(&self.database, chat_id);
    }

    pub fn get_all_chats(&self) -> Vec<Chat> {
        // Get all chats from the database
        crate::database::get_all_chats(&self.database).expect("Failed to get all chats")
    }

    pub fn handle_prompt(&mut self, prompt: Message) -> Vec<Message> {
        // Add the prompt the database
        crate::database::insert_message_into_chat(&self.database, self.current_chat.id, &prompt);

        // Add the prompt to the current chat in memory
        self.current_chat.messages.push(prompt);

        // Return the chat messages in memory
        self.current_chat.messages.clone()
    }

    pub fn handle_response(&mut self, response: Message) {
        // Add the response to the database
        crate::database::insert_message_into_chat(&self.database, self.current_chat.id, &response);

        // Add the reponse to the current chat in memory
        self.current_chat.messages.push(response);
    }
}
