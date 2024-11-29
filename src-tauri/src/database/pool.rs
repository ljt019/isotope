use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use std::path::Path;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn create_pool(path: &Path) -> Result<DbPool, r2d2::Error> {
    let manager = SqliteConnectionManager::file(path);
    Pool::builder().max_size(10).build(manager)
}
