#![allow(unused)]

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

// fn main() {

//     let args = Cli::parse();
//     let content = std::fs::read_to_string(&args.path)
//         .expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//         }
//     }

// }

use anyhow::{Context, Result};

fn main() -> Result<()> {
    let path = "src/main.rs";
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path))?;
    println!("file content: {}", content);
    Ok(())
}
