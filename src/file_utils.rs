use std::{
    fs::{self, File},
    io::{self, BufWriter, Write},
    path::Path,
    process,
};
use anyhow::{Context, Result};

pub fn parse_size(size_str: &str) -> Result<u64> {
    let size_str = size_str.trim().to_lowercase();
    let (num_str, suffix) = size_str.find(|c: char| c.is_alphabetic())
        .map(|i| (&size_str[..i], &size_str[i..]))
        .unwrap_or((&size_str, "b"));
    let num: u64 = num_str.parse().context("Invalid size number")?;
    
    match suffix {
        "k" | "kb" => Ok(num * 1024),
        "m" | "mb" => Ok(num * 1024 * 1024),
        "g" | "gb" => Ok(num * 1024 * 1024 * 1024),
        "b" | "" => Ok(num),
        _ => Err(anyhow::anyhow!("Invalid size suffix")),
    }
}

pub fn load_file_content(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("Failed to read file: {}", path.display()))
}

fn prompt_user(message: &str) -> io::Result<String> {
    print!("{}", message);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn get_alternative_filename(_: &str) -> io::Result<String> {
    loop {
        let input = prompt_user("Enter alternative filename: ")?;
        if input.is_empty() {
            println!("Filename cannot be empty. Please try again.");
            continue;
        }
        return Ok(input);
    }
}

fn prompt_file_action(path: &str) -> io::Result<FileAction> {
    loop {
        println!("\nFile '{}' already exists.", path);
        println!("Choose an action:");
        println!("  [o] Overwrite existing file");
        println!("  [n] Enter New filename");
        println!("  [c] Cancel operation");
        
        let response = prompt_user("Enter your choice (o/n/c): ")?.to_lowercase();
        match response.as_str() {
            "o" => return Ok(FileAction::Overwrite),
            "n" => return Ok(FileAction::NewName),
            "c" => return Ok(FileAction::Cancel),
            _ => println!("Invalid choice. Please select 'o', 'n', or 'c'"),
        }
    }
}

enum FileAction {
    Overwrite,
    NewName,
    Cancel,
}

pub fn get_output_writer(output_path: &str) -> Result<Box<dyn Write>> {
    if output_path == "-" {
        return Ok(Box::new(io::stdout()));
    }

    let mut current_path = output_path.to_string();
    loop {
        if Path::new(&current_path).exists() {
            match prompt_file_action(&current_path) {
                Ok(FileAction::Overwrite) => break,
                Ok(FileAction::NewName) => {
                    match get_alternative_filename(&current_path) {
                        Ok(new_path) => current_path = new_path,
                        Err(e) => return Err(anyhow::anyhow!("Failed to get alternative filename: {}", e)),
                    }
                },
                Ok(FileAction::Cancel) => {
                    println!("Operation cancelled by user.");
                    process::exit(0);
                },
                Err(e) => return Err(anyhow::anyhow!("Failed to get user input: {}", e)),
            }
        } else {
            break; // File doesn't exist, we can proceed
        }
    }

    let file = File::create(&current_path)
        .with_context(|| format!("Failed to create output file: {}", current_path))?;
    
    if current_path != output_path {
        println!("Writing to alternative file: {}", current_path);
    }
    
    Ok(Box::new(BufWriter::new(file)))
}