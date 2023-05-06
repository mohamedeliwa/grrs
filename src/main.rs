use clap::Parser;
use std::fs;
use std::io::{self, BufRead};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    // PathBuf is like a String but for file system paths that work cross-platform.
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    // the arguments that this program was started with (normally passed via the command line).
    // return an iterator
    let args = Cli::parse();
    let file = fs::File::open(&args.path).expect("couldn't open file");
    let mut reader = io::BufReader::new(file);
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(length) => {
                if length == 0 {
                    break;
                }
                if line.contains(&args.pattern) {
                    println!("{line}");
                }
            }
            Err(_) => {
                panic!("Couldn't read file")
            }
        };
    }
}
