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
        println!("no directory given");
        env::current_dir()?
    } else
    {
        PathBuf::from(matches.value_of("directory").unwrap())
    };

    println!("directory: {:?}", directory);
    
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
        },
        ("loc", Some(_config)) => {
            println!("all command");
            // Find the files in the directory.
            let files = get_file_paths(&[directory])?;

            process_code_statistics(&files)?;
            Ok(())
        },
        _ => Ok(()),
    }

}