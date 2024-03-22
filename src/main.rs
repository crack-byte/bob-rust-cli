use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::process;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Operation {
    Search,
    List,
}

/// A basic CLI tool to provide basic features
#[derive(Parser)]
#[command(author="crackbyte", version="0.0.1", about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    #[command(short_flag = 's')]
    Search {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'p', long)]
        pattern: String,
    },
    #[command(short_flag = 'c')]
    Count {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'p', long)]
        pattern: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    match args.command {
        None => { return Err("Missing valid arguments use bob -h to get help".into()); }
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
