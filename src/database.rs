use r2d2_sqlite::{SqliteConnectionManager};


pub fn connect(path: &str) -> SqliteConnectionManager {
    SqliteConnectionManager::file(path)
}