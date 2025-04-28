use std::path::{Path, PathBuf};
use pathdiff::diff_paths;

pub fn generate_tree_structure(files: &[PathBuf], root: &Path) -> String {
    let mut result = String::from("```\n");
    let files: Vec<_> = files
        .iter()
        .filter_map(|p| diff_paths(p, root).map(|p| p.to_string_lossy().into_owned()))
        .collect();

    for (i, file) in files.iter().enumerate() {
        let is_last = i == files.len() - 1;
        let prefix = if is_last { "└── " } else { "├── " };
        result.push_str(&format!("{}{}\n", prefix, file));
    }
    result.push_str("```\n");
    result
}