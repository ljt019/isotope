pub fn setup_database(config: &tauri::Config) -> rusqlite::Connection {
    // Get database directory
    let data_dir = tauri::api::path::app_data_dir(config).unwrap();
    let db_path = data_dir.join("database.db");

    // Check if databse exists
    let db_exists = db_path.exists();

    // Open connection (creates database if it doesn't exist)
    let connection = rusqlite::Connection::open(db_path).unwrap();

    // If database didn't exist, need to create tables
    if !db_exists {
        create_tables(&connection);
    }

    // Return connection to database
    connection
}

/*

Needed Tables:

Generation params


*/

pub fn create_tables(connection: &rusqlite::Connection) {
    connection
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
            [],
        )
        .unwrap();
}
