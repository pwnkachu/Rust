use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::io::{self, BufRead, BufReader};
use std::fs::File;
use colored::Colorize;

/// A maintainable grep clone built with Rust and Clap.
#[derive(Parser, Debug)]
#[command(
    name = "rsGrep",
    author,
    version,
    about = "A grep clone",
    long_about = None
)]
pub struct Cli {
    /// The string or pattern to search for
    #[arg(value_name = "PATTERN")]
    pub pattern: String,

    /// The file to search in
    #[arg(value_name = "FILE")]
    pub file_path: PathBuf,

    /// Ignore case distinctions in both the pattern and the input files
    #[arg(short = 'i', long)]
    pub ignore_case: bool,

    /// Invert the sense of matching, selecting non-matching lines
    #[arg(short = 'v', long)]
    pub invert_match: bool,

    /// Prefix each line of output with the 1-based line number
    #[arg(short = 'n', long)]
    pub line_number: bool,

    /// Suppress normal output; instead print a count of matching lines
    #[arg(short = 'c', long)]
    pub count: bool,
}


pub fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut file_path = PathBuf::new();

    if  cli.file_path.is_relative(){
        file_path = resolve_path(&cli.file_path)?;
    } else {
        file_path = cli.file_path.clone();
    }

    println!(" Debug: {} {}", cli.pattern, file_path.display());

    let f = File::open(file_path)?;
    let mut reader = BufReader::new(f);
    search_pattern(&cli, &mut reader)?;
    Ok(())
}

// Search a pattern inside a file's buffer
fn search_pattern(cli: &Cli, reader: &mut BufReader<File>) -> io::Result<()> {
    let mut match_count = 0; 

    // Uses both line index and line value
    for (line_index, line_result) in reader.lines().enumerate() {
        let line = line_result?;
        
        let index = line.find(&cli.pattern);
        
        if let Some(idx) = index {
            match_count += 1;
        
            if cli.count {
                continue; 
            }

            let match_end = idx + cli.pattern.len();
            let line_number = line_index + 1;

            let formatted_line = format!("{}{}{}", 
                &line[..idx], 
                &line[idx..match_end].red(), 
                &line[match_end..]
            );

            if cli.line_number {
                println!("{}:{}", line_number.to_string().green(), formatted_line);
            } else {
                println!("{}", formatted_line);
            }
        }
    }
    if cli.count {
        println!("{}", match_count);
    }

    Ok(())
}

// Will convert a relative path to an absolute one in respective to the caller dir
pub fn resolve_path(path_input: &PathBuf) -> io::Result<PathBuf> {
    let current_path = env::current_dir()?;
    let path = current_path.join(path_input);
    Ok(path)
}

