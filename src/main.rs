use anyhow::{Context, Result};
use clap::Parser;
use std::env::args;
use std::fs::File;
use std::io::{BufRead, BufReader};
/// ================================================================================================
/// A simple grep clone
/// ================================================================================================

/// The CLI struct is a simple struct that holds the pattern to search for and the path to the file to search in
#[derive(Parser)]
struct CLi {
    // The pattern to search for
    pattern: String,
    // PathBuf is a type provided by the standard library to represent paths that work cross-platform
    path: std::path::PathBuf,
}

fn main() {
    match read_cli_with_buffer() {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }
}

#[allow(dead_code)]
fn read_cli_with_buffer() -> Result<()> {
    let args: CLi = CLi::parse();
    let content = File::open(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;
    let buffered_content: BufReader<File> = BufReader::new(content);

    for line in buffered_content.lines() {
        let line: String = line.with_context(|| "Could not read line")?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn read_cli() {
    // Get the command line arguments
    // The first argument is the name of the pattern, the string to look for
    let pattern: String = args().nth(1).expect("no pattern given");

    // The second argument is the path to the file to read
    let path: String = args().nth(2).expect("no path given");

    let input_args = CLi {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
    println!(
        "pattern: {:?}, path: {:?}",
        input_args.pattern, input_args.path
    )
}
