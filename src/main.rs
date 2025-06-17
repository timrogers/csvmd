//! CSV to Markdown table converter CLI tool.

use clap::Parser;
use csvmd::error::Result;
use csvmd::{csv_to_markdown_streaming, Config};
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "csvmd")]
#[command(about = "Convert CSV to Markdown table")]
#[command(version)]
struct Args {
    /// Input CSV file (if not provided, reads from stdin)
    file: Option<PathBuf>,

    /// CSV delimiter character
    #[arg(short, long, default_value = ",")]
    delimiter: char,

    /// Treat first row as data, not headers
    #[arg(long)]
    no_headers: bool,

    /// Use streaming mode for large files (writes output immediately)
    #[arg(long)]
    stream: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config {
        has_headers: !args.no_headers,
        flexible: true,
        delimiter: args.delimiter as u8,
    };

    if args.stream {
        // Streaming mode: process row-by-row
        let input: Box<dyn Read> = match args.file {
            Some(path) => Box::new(File::open(path)?),
            None => Box::new(io::stdin()),
        };

        csv_to_markdown_streaming(input, io::stdout(), config)?;
    } else {
        // Standard mode: load all into memory then output
        let input: Box<dyn Read> = match args.file {
            Some(path) => Box::new(File::open(path)?),
            None => Box::new(io::stdin()),
        };

        let output = csvmd::csv_to_markdown(input, config)?;
        print!("{}", output);
    }

    Ok(())
}
