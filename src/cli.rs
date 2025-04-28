use clap::Parser;
use std::path::PathBuf;

/// A tool to generate LLM context prompts from directory contents
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(name = "promptify")]
#[command(about = "Generate LLM context prompts from directory contents")]
#[command(long_about = "A tool that scans a directory and generates a formatted prompt suitable for Large Language Models (LLMs). \
    It creates a structured output containing a directory tree and file contents, respecting gitignore rules and various filters.")]
pub struct Cli {
    /// Directory to scan (must be specified)
    #[arg(value_name = "DIRECTORY")]
    pub root_path: PathBuf,

    /// Output file path ("-" for stdout)
    #[arg(short, long, default_value = "prompt.md", value_name = "FILE")]
    pub output: String,

    /// Custom preamble text to add at the start of the output
    #[arg(long, value_name = "TEXT", conflicts_with = "preamble_file")]
    pub preamble: Option<String>,

    /// Path to a file containing the preamble
    #[arg(long, value_name = "FILE")]
    pub preamble_file: Option<PathBuf>,

    /// Custom postamble text to add at the end of the output
    #[arg(long, value_name = "TEXT", conflicts_with = "postamble_file")]
    pub postamble: Option<String>,

    /// Path to a file containing the postamble
    #[arg(long, value_name = "FILE")]
    pub postamble_file: Option<PathBuf>,

    /// Do not respect VCS ignore files (like .gitignore)
    #[arg(long)]
    pub no_gitignore: bool,

    /// Include binary files (metadata only)
    #[arg(long)]
    pub include_binaries: bool,

    /// Glob patterns for files to always include
    #[arg(long, value_name = "GLOB")]
    pub include: Vec<String>,

    /// Glob patterns for files/directories to explicitly exclude
    #[arg(long, value_name = "GLOB")]
    pub exclude: Vec<String>,

    /// Maximum file size (e.g., "1M", "512k")
    #[arg(long, value_name = "SIZE")]
    pub max_file_size: Option<String>,
}