use rusqlite::named_params;
use rusqlite::params;

pub fn setup_database(config: &tauri::Config) -> rusqlite::Connection {
    // Get database directory
    let data_dir = tauri::api::path::app_data_dir(config).unwrap();
    let db_path = data_dir.join("database.db");

    // Check if database exists
    let db_exists = db_path.exists();

    // Open connection (creates database if it doesn't exist)
    let connection = rusqlite::Connection::open(db_path).unwrap();

    // If database didn't exist, need to create tables
    if !db_exists {
        create_tables(&connection);
    }

    // Check if chats table exists
    let table_exists = connection
        .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='chats'")
        .unwrap()
        .query_map([], |row| Ok(row.get(0)?))
        .unwrap()
        .map(|table_name: rusqlite::Result<String>| table_name.unwrap())
        .next()
        .is_some();

    // If chats table doesn't exist, create it
    if !table_exists {
        create_tables(&connection);
    }

    // Return connection to database
    connection
}

#[derive(Debug)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub messages: Vec<crate::types::Message>,
}

pub fn create_tables(connection: &rusqlite::Connection) {
    connection
        .execute(
            "CREATE TABLE chats (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                created_at TEXT NOT NULL,
                messages TEXT NOT NULL
            )",
            [],
        )
        .unwrap();
}

pub fn insert_chat(connection: &rusqlite::Connection, name: String) -> rusqlite::Result<i64> {
    // Create timestamp in ISO 8601 format
    let timestamp = chrono::Local::now().to_rfc3339();

    // Initialize empty vec of Messages and serialize to JSON string
    let empty_messages = serde_json::to_string(&Vec::<crate::types::Message>::new()).unwrap();

    // Using named parameters for better readability
    connection.execute(
        "INSERT INTO chats (name, created_at, messages) VALUES (:name, :created_at, :messages)",
        named_params! {
            ":name": name,
            ":created_at": timestamp,
            ":messages": empty_messages,
        },
    )?;

    // Return the ID of the last inserted row
    Ok(connection.last_insert_rowid())
}

pub fn insert_message_into_chat(
    connection: &rusqlite::Connection,
    id: i64,
    new_messages: &[crate::types::Message],
) {
    // Get the chat
    let mut chat = get_chat(connection, id);

    // Add the new messages to the chat without overwriting the old ones
    chat.messages.push(new_messages.clone());

    // Update the chat in the database
    connection
        .execute(
            "UPDATE chats SET messages = ?1 WHERE id = ?2",
            params![serde_json::to_string(&chat.messages).unwrap(), id],
        )
        .unwrap();
}

pub fn get_chat(connection: &rusqlite::Connection, id: i64) -> Chat {
    let mut statement = connection
        .prepare("SELECT * FROM chats WHERE id = ?1")
        .unwrap();

    let chat_iter = statement
        .query_map(params![id], |row| {
            let messages_str: String = row.get(3)?;
            let messages = match serde_json::from_str(&messages_str) {
                Ok(msgs) => msgs,
                Err(_) => Vec::new(), // Return empty vec if parsing fails
            };

            Ok(Chat {
                id: row.get(0)?,
                name: row.get(1)?,
                created_at: row.get(2)?,
                messages,
            })
        })
        .unwrap();

    let x = chat_iter.map(|chat| chat.unwrap()).next().unwrap();
    x
}

pub fn get_chat_messages(connection: &rusqlite::Connection, id: i64) -> Vec<crate::types::Message> {
    let chat = get_chat(connection, id);
    chat.messages
}

pub fn get_all_chats(connection: &rusqlite::Connection) -> rusqlite::Result<Vec<Chat>> {
    let mut statement = connection.prepare("SELECT * FROM chats")?;

    let chats_iter = statement.query_map([], |row| {
        let messages_str: String = row.get(3)?;
        let messages = match serde_json::from_str(&messages_str) {
            Ok(msgs) => msgs,
            Err(_) => Vec::new(), // Return empty vec if parsing fails
        };

        Ok(Chat {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            messages,
        })
    })?;

    let mut chats = Vec::new();
    for chat in chats_iter {
        chats.push(chat?);
    }

    Ok(chats)
}

// Get the ID of the most recent chat
// Returns None if no chats exist
pub fn get_most_recent_chat(
    connection: &rusqlite::Connection,
) -> Result<Option<i64>, rusqlite::Error> {
    let mut statement = connection.prepare("SELECT id FROM chats ORDER BY id DESC LIMIT 1")?;

    let mut rows = statement.query([])?;

    // next() returns Option<Result<Row, Error>>
    match rows.next()? {
        Some(row) => Ok(Some(row.get::<_, i64>(0)?)),
        None => Ok(None),
    }
}
