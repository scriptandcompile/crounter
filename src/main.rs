use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    // Get the current directory.
    let current_path = env::current_dir()?;

    // Find the files in the directory.
    let path_entries = fs::read_dir(current_path)?;

    let mut files: Vec<PathBuf> = vec![];

    for entry in path_entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let file = path;

            files.push(file);
        } else if path.is_dir() {
            // walk the sub directories
        }
    }

    // Count the number of lines.
    for file in &files {
        let file_name = file.file_name().unwrap();
        let contents = fs::read_to_string(file)?;
        let line_count = contents.lines().count();

        // Display the statistics about the code lines.
        println!("file: {:?} - {:?} lines", file_name, line_count);
    }

    Ok(())
}
