use anyhow::Result;
use std::path::{Path, PathBuf};
use std::fs;

pub fn ensure_directory_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn get_app_data_directory() -> Result<PathBuf> {
    if let Some(home_dir) = dirs::home_dir() {
        let app_dir = home_dir.join("EdisonNote");
        ensure_directory_exists(&app_dir)?;
        Ok(app_dir)
    } else {
        Err(anyhow::anyhow!("Could not determine home directory"))
    }
}

pub fn get_notes_directory() -> Result<PathBuf> {
    let app_dir = get_app_data_directory()?;
    let notes_dir = app_dir.join("notes");
    ensure_directory_exists(&notes_dir)?;
    Ok(notes_dir)
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

pub fn get_file_extension(filename: &str) -> Option<&str> {
    Path::new(filename).extension()?.to_str()
}

pub fn read_file_to_string(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

pub fn write_string_to_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        ensure_directory_exists(parent)?;
    }
    Ok(fs::write(path, content)?)
}

pub fn list_markdown_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if dir.exists() && dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" || extension == "markdown" {
                        files.push(path);
                    }
                }
            }
        }
    }
    
    files.sort();
    Ok(files)
}

pub fn backup_file(path: &Path) -> Result<PathBuf> {
    let backup_path = path.with_extension(format!("{}.backup", 
        path.extension().and_then(|e| e.to_str()).unwrap_or("md")));
    
    fs::copy(path, &backup_path)?;
    Ok(backup_path)
}

pub fn get_file_size(path: &Path) -> Result<u64> {
    Ok(fs::metadata(path)?.len())
}

pub fn is_text_file(path: &Path) -> bool {
    if let Some(extension) = path.extension() {
        matches!(extension.to_str(), Some("md" | "txt" | "markdown" | "text"))
    } else {
        false
    }
}