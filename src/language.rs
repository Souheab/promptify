use std::path::Path;

pub fn infer_language(path: &Path) -> Option<&'static str> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| match ext.to_lowercase().as_str() {
            "rs" => "rust",
            "py" => "python",
            "js" => "javascript",
            "ts" => "typescript",
            "jsx" | "tsx" => "tsx",
            "cpp" | "cc" | "cxx" => "cpp",
            "c" => "c",
            "h" | "hpp" => "cpp",
            "go" => "go",
            "java" => "java",
            "kt" => "kotlin",
            "rb" => "ruby",
            "php" => "php",
            "cs" => "csharp",
            "swift" => "swift",
            "md" => "markdown",
            "json" => "json",
            "yaml" | "yml" => "yaml",
            "toml" => "toml",
            "xml" => "xml",
            "sql" => "sql",
            "sh" | "bash" => "bash",
            "ps1" => "powershell",
            "html" => "html",
            "css" => "css",
            "scss" | "sass" => "scss",
            "dockerfile" => "dockerfile",
            _ => "plaintext",
        })
}