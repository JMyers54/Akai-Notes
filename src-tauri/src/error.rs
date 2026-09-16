use  std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use crate::error::{Error,Result};
/*use = importar
#[derive(Debug,thiserror::Error)] = es un macro
es una instrucción para copila y derive es "genera
tu la implementacion de estos rasgos"
Debug = imprimir el error con {?} 
thiserror::Error = convertir el enum en un  error
que Rust entienda*/
const CURRENT_PROYECT_FILE: &str= "current_proyect.jsom";
const CURRENT_PROYECT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentProject {
    pub version: u32,
}
fn projects_file(app: &AppHandle) -> Result<PathBuf> {
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
    let project: Current_project = serde_json::from_str(
&std::fs::read_to_string(file)?)?;
    Ok(Some(project))
}

#[tauri::command]
pub fn set_current_project(app: AppHandle, path: string)
->Result<CurrentProject> {
    let file = project_file(&app)?;
    let project = CurrentProject {
        version: CURRENT_PROJECT_VERSDION,
        path: PathBuf::from(path),
    };
    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&file,serde_json::to_string_pretty(
    &project)?)?;
        Ok(project)
}