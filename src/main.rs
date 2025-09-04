//! CSV to Markdown table converter CLI tool.

use clap::{Parser, ValueEnum};
use csvmd::error::Result;
use csvmd::{csv_to_markdown_streaming, Config, HeaderAlignment};
use std::fs::File;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ClapAlignment {
    /// Left-aligned headers
    Left,
    /// Center-aligned headers
    Center,
    /// Center-aligned headers (British spelling)
    Centre,
    /// Right-aligned headers
    Right,
}

impl From<ClapAlignment> for HeaderAlignment {
    fn from(clap_align: ClapAlignment) -> Self {
        match clap_align {
            ClapAlignment::Left => HeaderAlignment::Left,
            ClapAlignment::Center | ClapAlignment::Centre => HeaderAlignment::Center,
            ClapAlignment::Right => HeaderAlignment::Right,
        }
    }
}

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

    /// Header alignment: left, center, or right
    #[arg(long, default_value = "left")]
    align: ClapAlignment,
}

/// A wrapper around stdin that shows a spinner after a timeout if it's interactive
struct InteractiveStdin {
    buffer: Vec<u8>,
    position: usize,
    initialized: bool,
}

impl InteractiveStdin {
    fn new() -> Self {
        Self {
            buffer: Vec::new(),
            position: 0,
            initialized: false,
        }
    }

    fn initialize_if_needed(&mut self) -> io::Result<()> {
        if self.initialized {
            return Ok(());
        }

        self.initialized = true;

        // Check if stdin is interactive (TTY)
        if !std::io::stdin().is_terminal() {
            // Not interactive, read all input immediately
            io::stdin().read_to_end(&mut self.buffer)?;
            return Ok(());
        }

        // Interactive session - implement timeout with spinner
        let (tx, rx) = mpsc::channel();
        let tx_for_thread = tx.clone();

        // Spawn thread to read from stdin
        thread::spawn(move || {
            let mut stdin_buffer = Vec::new();
            match io::stdin().read_to_end(&mut stdin_buffer) {
                Ok(_) => {
                    let _ = tx_for_thread.send(Ok(stdin_buffer));
                }
                Err(e) => {
                    let _ = tx_for_thread.send(Err(e));
                }
            }
        });

        // Wait 2 seconds for input
        thread::sleep(Duration::from_secs(2));

        // Check if input arrived
        match rx.try_recv() {
            Ok(Ok(data)) => {
                // Input arrived within 2 seconds
                self.buffer = data;
                return Ok(());
            }
            Ok(Err(e)) => {
                // Error occurred
                return Err(e);
            }
            Err(TryRecvError::Empty) => {
                // No input yet, show spinner
                self.show_spinner_and_wait(rx)?;
            }
            Err(TryRecvError::Disconnected) => {
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "Input thread disconnected",
                ));
            }
        }

        Ok(())
    }

    fn show_spinner_and_wait(
        &mut self,
        rx: Receiver<std::result::Result<Vec<u8>, io::Error>>,
    ) -> io::Result<()> {
        let _start_time = Instant::now();
        let mut message_shown = false;

        loop {
            // Show message once
            if !message_shown {
                eprint!("Waiting for input via stdin... (To read from a file, use `csvmd path/to/file.csv`.)");
                message_shown = true;
            }

            // Check for input
            match rx.try_recv() {
                Ok(Ok(data)) => {
                    // Clear message line
                    eprint!("\r{}\r", " ".repeat(85));
                    self.buffer = data;
                    return Ok(());
                }
                Ok(Err(e)) => {
                    eprint!("\r{}\r", " ".repeat(85));
                    return Err(e);
                }
                Err(TryRecvError::Empty) => {
                    // No input yet, continue waiting
                    thread::sleep(Duration::from_millis(100));
                }
                Err(TryRecvError::Disconnected) => {
                    eprint!("\r{}\r", " ".repeat(85));
                    return Err(io::Error::new(
                        io::ErrorKind::BrokenPipe,
                        "Input thread disconnected",
                    ));
                }
            }
        }
    }
}

impl Read for InteractiveStdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.initialize_if_needed()?;

        let remaining = self.buffer.len() - self.position;
        if remaining == 0 {
            return Ok(0); // EOF
        }

        let to_copy = buf.len().min(remaining);
        buf[..to_copy].copy_from_slice(&self.buffer[self.position..self.position + to_copy]);
        self.position += to_copy;

        Ok(to_copy)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    let config = Config {
        has_headers: !args.no_headers,
        flexible: true,
        delimiter: args.delimiter as u8,
        header_alignment: args.align.into(),
    };

    if args.stream {
        // Streaming mode
        match args.file {
            // For files, use seekable streaming to avoid buffering the entire input
            Some(path) => {
                let file = File::open(path)?;
                csvmd::csv_to_markdown_streaming_seekable(file, io::stdout(), config)?;
            }
            // For stdin or non-seekable, fall back to buffered streaming
            None => {
                let input: Box<dyn Read> = Box::new(InteractiveStdin::new());
                csv_to_markdown_streaming(input, io::stdout(), config)?;
            }
        }
    } else {
        // Standard mode: load all into memory then output
        let input: Box<dyn Read> = match args.file {
            Some(path) => Box::new(File::open(path)?),
            None => Box::new(InteractiveStdin::new()),
        };

        let output = csvmd::csv_to_markdown(input, config)?;
        print!("{}", output);
    }

    Ok(())
}
