use std::io;
use  std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use serde::Serializer;

#[derive(Debug, thiserror::Error)]
pub enum Error{
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Tauri error: {0}")]
    Tauri(#[from] tauri::Error),

    #[error("SQlite error {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("{0}")]
    Other(String),
}


impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok,S::Error>/*Result<S::Ok, S::Error>*/
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

const CURRENT_PROJECT_FILE: &str= "current_project.json";
const CURRENT_PROJECT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentProject {
    pub version: u32,
    pub path: String,
}
fn project_file(app: &AppHandle) -> Result<PathBuf> {
    Ok(app.path().app_data_dir()?.join(
CURRENT_PROJECT_FILE))
}

#[tauri::command]
pub fn get_current_project(app: AppHandle) ->
Result<Option<CurrentProject>> {
    let file = project_file(&app)?;
    if !file.exists() {
        return Ok(None);
    }
    let project: CurrentProject = serde_json::from_str(
&std::fs::read_to_string(file)?)?;
    if !Path ::new(&project.path).exists() {
        return Err(Error::NotFound(format!(
            "La carpeta del proyecto ya no existe: {}",
            project.path
        )));
    }
    Ok(Some(project))
}

#[tauri::command]
pub fn set_current_project(app: AppHandle, path: String)
->Result<CurrentProject> {
    let file = project_file(&app)?;
    let project = CurrentProject {
        version: CURRENT_PROJECT_VERSION,
        path,
    };
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&file,serde_json::to_string_pretty(
    &project)?)?;
        Ok(project)
}