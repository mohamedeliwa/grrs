use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Debug)]
#[derive(Parser)]
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
    let args =  Cli::parse();

}
