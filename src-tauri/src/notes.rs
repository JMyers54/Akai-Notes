use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::error::{Error,Result};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NoteNode {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub children: Vec<NoteNode>,
}

#[tauri::command]
pub fn list_notes(root: String) -> Result<Vec<NoteNode>> {
    let base = PathBuf::from(&root);
    if !base.is_dir() {
        return Err(Error::NotFound(format!(
            "La carpeta del proyecto no existe: {root}"
        )));
    }
    scan(&base, &base)
}

fn scan(base: &Path, dir: &Path) -> Result<Vec<NoteNode>>{
    let mut nodes = Vec::new();
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('-') {
            continue;
        }
        let relative = path
            .strip_prefix(base)
            .unwrap_or(&base)
            .to_string_lossy()
            .replace('\\',"/");

        if path.is_dir() {
            nodes.push(NoteNode {
                name,
                path: relative,
                is_dir: true,
                children: scan(base, &path)?,
            });
        } else if path
            .extension()
            .map(|ext| ext.eq_ignore_ascii_case("md"))
            .unwrap_or(false)
            {
                nodes.push(NoteNode {
                    name,
                    path: relative,
                    is_dir: false,
                    children: Vec::new(),
                });
            }
    } 
    nodes.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
    Ok(nodes)
}

#[tauri::command]
pub fn read_note(root: String, relative_path: String) ->
Result<String> {
    let root_canon = std::fs::canonicalize(&root)?;
    let file_canon = std::fs::canonicalize(Path::new(&root).join(&relative_path))?;
    if !file_canon.starts_with(&root_canon) {
        return Err(Error::Other("Ruta fuera del proyecto".into()));
    }
            Ok(std::fs::read_to_string(file_canon)?)
}


