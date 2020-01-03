extern crate clap;
extern crate ignore;

use std::env;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

use clap::{App, AppSettings, Arg, SubCommand};
use ignore::WalkBuilder;

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

    let mut non_code_files = vec![];
    let mut non_code_header_flag = false;
    
    let mut code_file_count = 0;

    let mut code_lines_total = 0;
    let mut whitespace_lines_total = 0;

    for file_name in code_files {
        let contents = match fs::read_to_string(&file_name) {
            Ok(content) => content,
            Err(_message) => {
                non_code_files.push(file_name);
                continue;
            }
        };

        let mut code_lines = 0;
        let mut whitespace_lines = 0;

        for line in contents.lines() {
            if line.trim().is_empty() {
                whitespace_lines += 1;
            } else {
                code_lines += 1;
            }
        }
        
        code_lines_total += code_lines;
        whitespace_lines_total += whitespace_lines;
        
        code_file_count += 1;

        if non_code_header_flag == false {
            non_code_header_flag = true;
            println!("Code files:");
            println!("-----------------------------------");
        }

        println!(
            "{} - {} code lines, {} whitespace lines",
            &file_name.display(), code_lines, whitespace_lines
        );
    }

    let non_code_file_count = non_code_files.len();

    if non_code_file_count != 0 {
        println!("");
        println!("Non-code files:");
        println!("-----------------------------------");
    }

    for file_name in non_code_files {
        println!("file: {}", &file_name.display());
    }

    if code_files.len() != 0 {
        println!("");
        println!("Totals:");
        println!("-----------------------------------");
        println!("{} code files, {} non-code files", code_file_count, non_code_file_count);
        println!("Code lines: {}", code_lines_total);
        println!("Whitespace lines: {}", whitespace_lines_total);
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("directory")
                .takes_value(true)
                .short("d")
                .help("Directory to process."),
        )
        .subcommand(
            SubCommand::with_name("all")
                .setting(AppSettings::Hidden)
                .about("Run all metric commands.")
                .arg(
                    Arg::with_name("directory")
                        .takes_value(true)
                        .short("d")
                        .help("Directory to process."),
                ),
        )
        .subcommand(
            SubCommand::with_name("loc")
                .about("Lines of code metric command.")
                .arg(
                    Arg::with_name("directory")
                        .takes_value(true)
                        .short("d")
                        .help("Directory to process."),
                ),
        )
        .get_matches();

    let arg_count = env::args().count();

    let directory = if matches.value_of("directory").is_none() {
        env::current_dir()?
    } else {
        PathBuf::from(matches.value_of("directory").unwrap())
    };

    if arg_count == 0 || arg_count == 1 {
        // Find the files in the directory.
        let files = get_file_paths(&[directory])?;

        process_code_statistics(&files)?;

        return Ok(());
    }

    match matches.subcommand() {
        ("all", Some(_config)) => {
            println!("all command");
            // Find the files in the directory.
            let files = get_file_paths(&[directory])?;

            process_code_statistics(&files)?;
            Ok(())
        }
        ("loc", Some(_config)) => {
            println!("all command");
            // Find the files in the directory.
            let files = get_file_paths(&[directory])?;

            process_code_statistics(&files)?;
            Ok(())
        }
        _ => Ok(()),
    }
}
