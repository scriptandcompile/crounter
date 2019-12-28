extern crate fs_extra;

use std::error::Error;
use std::env;
use std::fs;

use fs_extra::dir::get_dir_content;

fn main() -> Result<(), Box<dyn Error>> {
    // Get the current directory.
    let current_path = &env::current_dir()?;

    // Find the files in the directory.
    let dir_content = get_dir_content(current_path)?;
    
    for file_name in dir_content.files {

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
