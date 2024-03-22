use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::process;

use clap::{Parser, Subcommand};
use zip::write::FileOptions;
use zip::ZipWriter;

/// A basic CLI tool to provide basic features
#[derive(Parser)]
#[command(author="crackbyte", version="0.0.1", about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug, Clone)]
enum Command {
    /// search a pattern in a file
    #[command(short_flag = 's')]
    Search {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'p', long)]
        pattern: String,
    },
    /// count a pattern occurrence in a file
    #[command(short_flag = 'c')]
    Count {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'p', long)]
        pattern: String,
    },
    /// compress a file
    #[command(short_flag = 'z')]
    Compress {
        #[clap(short = 'f', long)]
        filename: String,
        #[clap(short = 'o', long)]
        output: String,
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

        Some(Command::Compress { filename,output }) => {
            match compress_file(&filename, &output) {
                Ok(_) => println!("Compression successful!"),
                Err(e) => println!("Error: {}", e),
            }
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
        println!("Found {count} occurrences.")
    } else {
        println!("Nothing found")
    }
}
fn compress_file(src_file: &str, archive_file: &str) -> Result<(), zip::result::ZipError> {
    let mut src_f = File::open(src_file)?;
    let dest_f = File::create(archive_file)?;

    let mut zip = ZipWriter::new(dest_f);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2); // No compression for simplicity

    zip.start_file(src_file, options)?;
    std::io::copy(&mut src_f, &mut zip)?;

    zip.finish()?;
    Ok(())
}
