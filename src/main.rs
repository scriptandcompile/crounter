extern crate clap;
extern crate ignore;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use ignore::WalkBuilder;
use clap::{App, AppSettings, Arg, SubCommand};

// Need to to fix this in the future to return a collection of
// search path results and errors. That way we can provide partial
// results if a file/directory is blocked/missing and a log
// of unaccessable or non-processable files/directories.
// For now, we simply bail currently on error.
fn get_file_paths(search_paths: &[PathBuf]) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mut file_paths = vec![];

    for search_path in search_paths {
        let walker = WalkBuilder::new(&search_path).build();
        for file in walker {
            let dir_entry = file?;
            if dir_entry.metadata()?.is_dir() {
                continue;
            }
            file_paths.push(dir_entry.into_path());
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
}ignore::