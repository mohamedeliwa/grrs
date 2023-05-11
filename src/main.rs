use anyhow::{Context, Ok, Result};
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

fn main() -> Result<()> {
    // the arguments that this program was started with (normally passed via the command line).
    // return an iterator
    let args = Cli::parse();
    let file = fs::File::open(&args.path)
        .with_context(|| format!("could not open file `{:?}`", args.path))?;
    let mut reader = io::BufReader::new(file);
    loop {
        let mut line = String::new();
        let length = reader
            .read_line(&mut line)
            .with_context(|| format!("could not read file `{:?}`", args.path))?;
        if length == 0 {
            break;
        }
        match_a_line(&args.pattern, &line, &mut io::stdout())?;
    }
    Ok(())
}

fn match_a_line(pattern: &str, line: &str, writer: &mut impl std::io::Write) -> Result<()> {
    if line.contains(pattern) {
        writeln!(writer, "{line}").with_context(|| format!("could not write a match"))?;
    }
    Ok(())
}

#[test]
fn should_match_a_line() {
    let mut result: Vec<u8> = Vec::new();
    match_a_line("lorem", "lorem ipsum\n", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n\n");
}
