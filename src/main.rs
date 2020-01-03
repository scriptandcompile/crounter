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
        if !search_path.exists() {
            println!("Unable to find directory: {}", search_path.display());
            continue;
        }

        let walker = WalkBuilder::new(&search_path).build();
        for file in walker {
            let dir_entry = match file {
                Ok(entry) => entry,
                Err(er) => {
                    println!("err is devine: {:?}", er);
                    continue;
                }
            };

            if dir_entry.metadata()?.is_dir() {
                continue;
            }
            file_paths.push(dir_entry.into_path());
        }
    }

    Ok(file_paths)
}

pub struct LineStatistics {
    pub total: u64,
    pub whitespace: u64,
    pub code: u64,
}

pub struct FileStatistics {
    pub file_path: PathBuf,
    pub loc: LineStatistics,
}

pub struct ProjectStatistics {
    pub code_files: Vec<FileStatistics>,
    pub non_code_files: Vec<PathBuf>,
}

fn display_statistics(project_stats: &ProjectStatistics) {
    for (index, code_file) in project_stats.code_files.iter().enumerate() {
        if index == 0 {
            println!("Code files:");
            println!("-----------------------------------");
        }

        println!(
            "{} - {} code lines, {} whitespace lines",
            &code_file.file_path.display(),
            code_file.loc.code,
            code_file.loc.whitespace
        );
    }

    for (index, non_code_file) in project_stats.non_code_files.iter().enumerate() {
        if index == 0 {
            println!("");
            println!("Non-code files:");
            println!("-----------------------------------");
        }

        println!("file: {}", &non_code_file.display());
    }

    let code_file_count = project_stats.code_files.len();
    let non_code_file_count = project_stats.non_code_files.len();
    let code_lines_total = project_stats
        .code_files
        .iter()
        .fold(0, |accumulator, code_file| accumulator + code_file.loc.code);
    let whitespace_lines_total = project_stats
        .code_files
        .iter()
        .fold(0, |accumulator, code_file| {
            accumulator + code_file.loc.whitespace
        });

    if code_file_count != 0 {
        println!("");
        println!("Totals:");
        println!("-----------------------------------");
        println!(
            "{} code files, {} non-code files",
            code_file_count, non_code_file_count
        );
        println!("Code lines: {}", code_lines_total);
        println!("Whitespace lines: {}", whitespace_lines_total);
    }
}

fn process_code_statistics(code_files: &[PathBuf]) -> Result<ProjectStatistics, Box<dyn Error>> {
    let mut project_stats = ProjectStatistics {
        code_files: vec![],
        non_code_files: vec![],
    };

    for file_path in code_files {
        let contents = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(_message) => {
                project_stats.non_code_files.push(file_path.to_path_buf());
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

        project_stats.code_files.push(FileStatistics {
            file_path: file_path.to_path_buf(),
            loc: LineStatistics {
                total: whitespace_lines + code_lines,
                whitespace: whitespace_lines,
                code: code_lines,
            },
        });
    }

    Ok(project_stats)
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

        let project_stats = process_code_statistics(&files)?;
        display_statistics(&project_stats);

        return Ok(());
    }

    match matches.subcommand() {
        ("all", Some(_config)) => {
            // Find the files in the directory.
            let files = get_file_paths(&[directory])?;

            let project_stats = process_code_statistics(&files)?;
            display_statistics(&project_stats);

            Ok(())
        }
        ("loc", Some(_config)) => {
            // Find the files in the directory.
            let files = get_file_paths(&[directory])?;

            let project_stats = process_code_statistics(&files)?;
            display_statistics(&project_stats);

            Ok(())
        }
        _ => {
            // Find the files in the directory.
            let files = get_file_paths(&[directory])?;

            let project_stats = process_code_statistics(&files)?;
            display_statistics(&project_stats);

            Ok(())
        }
    }
}
