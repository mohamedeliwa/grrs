use std::env;

#[derive(Debug)]
struct Cli {
    pattern: String,
    // PathBuf is like a String but for file system paths that work cross-platform.
    path: std::path::PathBuf,
}

fn main() {
    // the arguments that this program was started with (normally passed via the command line).
    // return an iterator
    let mut args = env::args();
    
    let pattern = args.nth(1).expect("no pattern given");
    let path = args.nth(2).expect("no path given");

}
