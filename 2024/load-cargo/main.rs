use std::collections::HashSet;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let base_dir = std::env::current_dir()?; // Start in the current directory
    let cargo_toml_path = base_dir.join("Cargo.toml");
    let cargo_ignore_path = base_dir.join(".cargoignore");

    // // Ensure the `Cargo.toml` file exists
    // if !cargo_toml_path.exists() {
    //     eprintln!("Error: Cargo.toml not found in the current directory.");
    //     return Ok(());
    // }

    // Find all Rust files in the directory recursively
    let rust_files = find_rust_files(&base_dir)?;

    let ignores: HashSet<String> = if cargo_ignore_path.exists() {
        let file = fs::File::open(&cargo_ignore_path)?;
        let reader = io::BufReader::new(file);
        reader.lines().filter_map(|line| line.ok()).collect()
    } else {
        HashSet::new()
    };

    let new_cargo_toml = generate_cargo_toml(&base_dir, &rust_files, &ignores)?;

    // println!("{:?}", new_cargo_toml);
    fs::write(cargo_toml_path, new_cargo_toml)?;

    // Update `Cargo.toml` with the binary targets
    // update_cargo_toml(&cargo_toml_path, &rust_files)?;

    println!("Cargo.toml has been updated successfully.");
    Ok(())
}

fn find_rust_files(base_dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut rust_files = Vec::new();
    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            rust_files.extend(find_rust_files(&path)?); // Recurse into directories
        } else if let Some(ext) = path.extension() {
            if ext == "rs" {
                rust_files.push(path);
            }
        }
    }
    Ok(rust_files)
}

fn generate_cargo_toml(base_dir: &Path, rust_files: &[PathBuf], ignores: &HashSet<String>) -> io::Result<String> {
    // Start with the basic package definition
    let mut cargo_toml = String::from("[package]\n");
    cargo_toml.push_str("name = \"aoc2024\"\n");
    cargo_toml.push_str("version = \"0.1.0\"\n");
    cargo_toml.push_str("edition = \"2021\"\n\n");

    cargo_toml.push_str("[dependencies]\n");
    cargo_toml.push_str("regex = \"1.11.1\"\n\n");
    
    // Add binary targets
    for file in rust_files {
        if let Some(name) = extract_binary_name(file) {
            if ignores.contains(&name) {
                continue;
            }
            let relative_path = file.strip_prefix(base_dir)
                .expect("Failed to get relative path")
                .display()
                .to_string();
            cargo_toml.push_str(&format!(
                "[[bin]]\nname = \"{}\"\npath = \"{}\"\n\n",
                name,
                relative_path
            ));
        }
    }

    Ok(cargo_toml)
}

fn extract_binary_name(file: &Path) -> Option<String> {
    file.parent()
        .and_then(|parent| parent.file_name()) // Get the parent directory name
        .and_then(|dir_name| dir_name.to_str()) // Convert it to a &str
        .and_then(|dir_name| dir_name.split_whitespace().next()) // Take the first part (e.g., "49")
        .map(String::from)
}