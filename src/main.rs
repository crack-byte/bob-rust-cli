use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::process;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Operation {
    Search,
    List,
}

/// A basic CLI tool to greet the user
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    done: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    Search {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'p', long)]
        pattern: String,
    },
    Count {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'p', long)]
        pattern: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Check if at least one argument (-p or -f) is provided
    match args.done {
        None => { println!("none") }
        Some(Command::Search { filename, pattern }) => {
            search(filename, pattern)
        }
        Some(Command::Count { filename, pattern }) => {
            count(filename, pattern)
        }
    }
    Ok(())
}

fn search(filename: String, pattern: String) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    let lines: Lines<BufReader<File>> = reader.lines();

    // Search for the pattern in each line
    for (number, line) in lines.enumerate() {
        if let Ok(line_str) = line {
            if line_str.contains(&pattern) {
                println!("Line:{number} {line_str}");
            }
        }
    }
}

fn count(filename: String, pattern: String) {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            process::exit(1);
        }
    };
    let reader = BufReader::new(file);
    let lines: Lines<BufReader<File>> = reader.lines();
    let mut count: usize = 0;
    // Search for the pattern in each line
    for line in lines {
        if let Ok(line_str) = line {
            if line_str.contains(&pattern) {
                for word in line_str.split_whitespace() {
                    if word.contains(&pattern) { count += 1; }
                }
            }
        }
    }
    if count > 0 {
        println!("Found {count} occurrences")
    } else {
        println!("Nothing found")
    }
}
