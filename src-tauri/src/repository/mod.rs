use rusqlite::Result;

use crate::models::Task;

pub mod sqlite;

pub trait TaskRepository: Send + Sync {
    fn add(&self, title: String) -> Result<Task>;
    fn list(&self) -> Result<Vec<Task>>;
    fn delete(&self, id: i64) -> Result<()>;
}
