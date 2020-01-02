extern crate clap;
extern crate fs_extra;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::{App, Arg, SubCommand};
use fs_extra::dir::get_dir_content;

// Need to to fix this in the future to return a collection of
// search path results and errors. That way we can provide partial
// results if a file/directory is blocked/missing and a log
// of unaccessable or non-processable files/directories.
// For now, we simply bail currently on error.
fn get_file_paths(search_paths: &[PathBuf]) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut file_paths = vec![];

    for search_path in search_paths {
        let dir_content = get_dir_content(search_path)?;
        for file in dir_content.files {
            file_paths.push(PathBuf::from(file));
        }
    }

    Ok(file_paths)
}

fn process_code_statistics(code_files: &[PathBuf]) -> Result<(), Box<dyn Error>> {
    for file_name in code_files {
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

fn main() -> Result<(), Box<dyn Error>> {
    let _matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .get_matches();

    let arg_count = env::args().count();

    if arg_count == 0 || arg_count == 1 {
        let current_path = env::current_dir()?;

        // Find the files in the directory.
        let files = get_file_paths(&[current_path])?;
        process_code_statistics(&files)?;
    }

    Ok(())
}
