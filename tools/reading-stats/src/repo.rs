use std::error::Error;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn find_repo_root(start: &Path) -> Result<PathBuf, Box<dyn Error>> {
    for candidate in start.ancestors() {
        if candidate.join("content").is_dir() && candidate.join("templates").is_dir() {
            return Ok(candidate.to_path_buf());
        }
    }

    Err("could not locate repo root from current directory".into())
}

pub fn markdown_paths(content_dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(content_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(walkdir::DirEntry::into_path)
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("md"))
        .filter(|path| path.file_name().and_then(|name| name.to_str()) != Some("_index.md"))
        .collect()
}

pub fn relative_content_path(content_dir: &Path, path: &Path) -> Result<String, Box<dyn Error>> {
    Ok(path
        .strip_prefix(content_dir)?
        .to_string_lossy()
        .replace('\\', "/"))
}
