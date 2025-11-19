use crate::models::{Priority, Status, Task};
use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use directories::ProjectDirs;
use rusqlite::{params, Connection};
use std::fs;

pub struct TaskRepository {
    conn: Connection,
}

impl TaskRepository {
    pub fn init() -> Result<Self> {
        let project_dirs = ProjectDirs::from("com", "flow", "flow")
            .context("Could not determine project directories")?;
        let data_dir = project_dirs.data_dir();
        fs::create_dir_all(data_dir)?;

        let db_path = data_dir.join("flow.db");
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                due_date TEXT,
                project_path TEXT,
                created_at TEXT NOT NULL,
                tags TEXT
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn add_task(&self, task: &Task) -> Result<i64> {
        let tags_str = serde_json::to_string(&task.tags)?;
        self.conn.execute(
            "INSERT INTO tasks (title, description, status, priority, due_date, project_path, created_at, tags)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                task.title,
                task.description,
                task.status.to_string(),
                task.priority.to_string(),
                task.due_date.map(|d| d.to_rfc3339()),
                task.project_path,
                task.created_at.to_rfc3339(),
                tags_str
            ],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn get_tasks(&self, project_filter: Option<&str>) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, title, description, status, priority, due_date, project_path, created_at, tags 
             FROM tasks 
             WHERE ?1 IS NULL OR project_path = ?1"
        )?;

        let task_iter = stmt.query_map(params![project_filter], |row| {
            let status_str: String = row.get(3)?;
            let priority_str: String = row.get(4)?;
            let due_date_str: Option<String> = row.get(5)?;
            let created_at_str: String = row.get(7)?;
            let tags_str: String = row.get(8)?;

            Ok(Task {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                description: row.get(2)?,
                status: Status::from(status_str),
                priority: Priority::from(priority_str),
                due_date: due_date_str.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&Local))),
                project_path: row.get(6)?,
                created_at: DateTime::parse_from_rfc3339(&created_at_str)
                    .map(|dt| dt.with_timezone(&Local))
                    .unwrap_or_else(|_| Local::now()),
                tags: serde_json::from_str(&tags_str).unwrap_or_default(),
            })
        })?;

        let mut tasks = Vec::new();
        for task in task_iter {
            tasks.push(task?);
        }
        Ok(tasks)
    }

    pub fn complete_task(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![Status::Done.to_string(), id],
        )?;
        Ok(())
    }

    pub fn delete_task(&self, id: i64) -> Result<()> {
        self.conn.execute("DELETE FROM tasks WHERE id = ?1", params![id])?;
        Ok(())
    }
}
