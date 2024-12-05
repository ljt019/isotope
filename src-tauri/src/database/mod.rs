pub mod pool;

use crate::database::pool::DbPool;
use rusqlite::{named_params, params};

pub fn setup_database(pool: &DbPool) {
    let connection = pool.get().expect("Failed to get connection from pool");

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
        create_tables(pool);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Chat {
    pub id: i64,
    pub name: String,
    pub created_at: String,
    pub messages: Vec<crate::models::chat_manager::Message>,
}

pub fn create_tables(pool: &DbPool) {
    let connection = pool.get().expect("Failed to get connection from pool");

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

pub fn insert_chat(pool: &DbPool, name: String) -> rusqlite::Result<i64> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let timestamp = chrono::Local::now().to_rfc3339();
    let empty_messages =
        serde_json::to_string(&Vec::<crate::models::chat_manager::Message>::new()).unwrap();

    conn.execute(
        "INSERT INTO chats (name, created_at, messages) VALUES (:name, :created_at, :messages)",
        named_params! {
            ":name": name,
            ":created_at": timestamp,
            ":messages": empty_messages,
        },
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn insert_message_into_chat(
    pool: &DbPool,
    id: i64,
    new_message: &crate::models::chat_manager::Message,
) -> rusqlite::Result<()> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut chat = get_chat(pool, id)?;

    chat.messages.push(new_message.clone());

    conn.execute(
        "UPDATE chats SET messages = ?1 WHERE id = ?2",
        params![serde_json::to_string(&chat.messages).unwrap(), id],
    )?;

    Ok(())
}

pub fn get_chat(pool: &DbPool, id: i64) -> rusqlite::Result<Chat> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut statement = conn.prepare("SELECT * FROM chats WHERE id = ?1")?;

    statement.query_row(params![id], |row| {
        let messages_str: String = row.get(3)?;
        let messages = serde_json::from_str(&messages_str).unwrap_or_default();

        Ok(Chat {
            id: row.get(0)?,
            name: row.get(1)?,
            created_at: row.get(2)?,
            messages,
        })
    })
}

#[allow(dead_code)]
pub fn get_chat_messages(
    pool: &DbPool,
    id: i64,
) -> rusqlite::Result<Vec<crate::models::chat_manager::Message>> {
    let chat = get_chat(pool, id)?;
    Ok(chat.messages)
}

pub fn get_all_chats(pool: &DbPool) -> rusqlite::Result<Vec<Chat>> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut statement = conn.prepare("SELECT * FROM chats")?;

    let chats_iter = statement.query_map([], |row| {
        let messages_str: String = row.get(3)?;
        let messages = serde_json::from_str(&messages_str).unwrap_or_default();

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

pub fn get_most_recent_chat(pool: &DbPool) -> rusqlite::Result<Option<i64>> {
    let conn = pool.get().expect("Failed to get connection from pool");
    let mut statement = conn.prepare("SELECT id FROM chats ORDER BY id DESC LIMIT 1")?;

    let mut rows = statement.query([])?;

    match rows.next()? {
        Some(row) => Ok(Some(row.get::<_, i64>(0)?)),
        None => Ok(None),
    }
}
