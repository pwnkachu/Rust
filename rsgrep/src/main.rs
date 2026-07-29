use rsgrep::{Cli, resolve_path, search_pattern}; 

use clap::Parser;
use std::fs::File;
use std::io::{self, BufReader};
use std::path::PathBuf;

pub fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let mut file_path = PathBuf::new();

    // 2. Risoluzione del path
    if cli.file_path.is_relative() {
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