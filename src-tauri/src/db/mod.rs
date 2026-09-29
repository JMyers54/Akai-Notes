
use std::sync::{Mutex, MutexGuard};
use rusqlite::Connection;
use tauri::{Manager, State};

use crate::error::{Error,Result};

pub struct Database(pub Mutex<Connection>);

pub fn setup(app: &tauri::App) ->Result<()> {
    let dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&dir)?;
    let conn = Connection::open(dir.join("akai-notes.db"))?;
    conn.pragma_update(None, "foreign_keys", "ON")?;
    conn.execute_batch(include_str!("schema.sql"))?;
    app.manage(Database(Mutex::new(conn)));
    Ok(())
}

pub fn conn<'a>(state: & 'a State<'_, Database>) ->
Result<MutexGuard<'a, Connection>> {
    state
        .0
        .lock()
        .map_err(|_| Error::Other("Database lock poisoned".into()))
}
