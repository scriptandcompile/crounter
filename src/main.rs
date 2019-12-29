extern crate fs_extra;

use std::error::Error;
use std::path::{PathBuf};
use std::env;
use std::fs;

use fs_extra::dir::get_dir_content;

// Need to to fix this in the future to return a collection of
// search path results and errors. That way we can provide partial
// results if a file/directory is blocked/missing and a log
// of unaccessable or non-processable files/directories. 
// For now, we simply bail currently on error.
fn get_file_paths(search_paths: &[PathBuf]) -> Result<Vec<String>, Box<dyn Error>> {
    let mut file_paths = vec!();

    for search_path in search_paths {
        let mut dir_content = get_dir_content(search_path)?;
        file_paths.append(&mut dir_content.files);
    }

    Ok(file_paths)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Get the current directory.
    let current_path = env::current_dir()?;

    // Find the files in the directory.
    let files = get_file_paths(&[current_path])?;
    
    for file_name in files {

        let contents = fs::read_to_string(&file_name)?;
        let mut code_line = 0;
        let mut whitespace_line = 0;

        for line in contents.lines() {
            if line.trim().is_empty() {
                whitespace_line += 1;
            } else {
                code_line += 1;
            }
        }
        
        println!(
            "file: {:?} - {:?} code lines, {:?} whitespace lines",
            &file_name, code_line, whitespace_line
        );
    }

    Ok(())
}
