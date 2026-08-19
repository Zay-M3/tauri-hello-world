use std::sync::{Arc, Mutex};

use rusqlite::{params, Connection, Result};

use super::TaskRepository;
use crate::models::Task;

pub struct SqliteTaskRepository {
    pub conn: Arc<Mutex<Connection>>,
}

impl TaskRepository for SqliteTaskRepository {
    fn add(&self, title: String) -> Result<Task> {
        let conn = self.conn.lock().expect("mutex poisoned");
        conn.execute("INSERT INTO tasks (title) VALUES (?1)", params![title])?;
        let id = conn.last_insert_rowid();
        let created_at = conn.query_row(
            "SELECT created_at FROM tasks WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        Ok(Task { id, title, created_at })
    }

    fn list(&self) -> Result<Vec<Task>> {
        let conn = self.conn.lock().expect("mutex poisoned");
        let mut stmt =
            conn.prepare("SELECT id, title, created_at FROM tasks ORDER BY id DESC")?;
        let tasks = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    created_at: row.get(2)?,
                })
            })?
            .collect::<Result<Vec<_>>>()?;
        Ok(tasks)
    }

    fn delete(&self, id: i64) -> Result<()> {
        let conn = self.conn.lock().expect("mutex poisoned");
        conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }
}
