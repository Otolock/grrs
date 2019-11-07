use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();
    println!(
        "Searching for {} in the file {:?}.",
        args.pattern,
        args.path.to_str()
    );

    let file = File::open(&args.path).expect("File not found or cannot be opened");
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        if line.contains(&args.pattern) {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            writeln!(handle, "{:?}", line)?;
        }
    }

    Ok(())
}
