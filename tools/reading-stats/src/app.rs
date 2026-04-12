use std::collections::BTreeMap;
use std::error::Error;
use std::fs;

use crate::markdown::compute_stats;
use crate::repo::{find_repo_root, markdown_paths, relative_content_path};

pub fn run() -> Result<(), Box<dyn Error>> {
    let repo_root = find_repo_root(&std::env::current_dir()?)?;
    let content_dir = repo_root.join("content");
    let output_path = repo_root.join("data").join("reading_stats.json");

    let mut stats = BTreeMap::new();

    for path in markdown_paths(&content_dir) {
        let relative_path = relative_content_path(&content_dir, &path)?;
        stats.insert(relative_path, compute_stats(&path)?);
    }

    fs::create_dir_all(output_path.parent().expect("output path has a parent"))?;
    fs::write(output_path, serde_json::to_string_pretty(&stats)? + "\n")?;

    Ok(())
}
