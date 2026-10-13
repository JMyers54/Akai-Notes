use serde::Serialize;
use tauri::State;

use crate::db::{conn, Database};
use crate::error::Result;

#[derive(Debug, Serialize)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub path: String,
    pub created_at:i64,
    pub last_opened: i64,
}

#[tauri::command]
pub fn list_projects(state: State<'_, Database>) ->
Result<Vec<Project>> {
    let conn = conn(&state)?;
    let mut stmt = conn.prepare(
        "SELECT id, name, path, created_at, last_opened
        FROM projects ORDER BY last_opened DESC",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            path: row.get(2)?,
            created_at: row.get(3)?,
            last_opened: row.get(4)?,
        })
    })?;
    let mut projects = Vec::new();
    for p in rows {
        projects.push(p?);
    }
    Ok(projects)
}