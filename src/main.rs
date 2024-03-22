use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::process;

use clap::{Parser, ValueEnum};

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Operation {
    Search,
    List,
}

/// A basic CLI tool to greet the user
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'g', long)]
    greet: Option<String>,
    #[arg(short = 'o', long)]
    operation: Option<Operation>,
    #[arg(short = 'f', long)]
    file: Option<String>,
    #[arg(short = 'p', long)]
    pattern: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // Check if at least one argument (-p or -f) is provided

    match args.operation {
        Some(Operation::Search) => {
            if let Some(filename) = args.file {
                if let Some(pattern) = args.pattern {
                    search(filename, pattern);
                } else {
                    return Err("Missing pattern argument for search operation".into());
                }
            } else {
                return Err("Missing file for search operation".into());
            }
        }
        Some(Operation::List) => {
            // Implement List functionality here
            println!("Listing functionality is not yet implemented");
        }
        None => {
            println!("Hello");
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
    for line in lines {
        if let Ok(line_str) = line {
            if line_str.contains(&pattern) {
                println!("{}", line_str);
            }
        }
    }
}
