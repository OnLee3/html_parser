use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// markdown to html.
struct Args {
    /// 입력 파일(들)
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,
}

fn open(filename: &str) -> Result<BufReader<File>> {
    Ok(BufReader::new(File::open(filename)?))
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                for line in file.lines() {
                    let line = line?;
                    println!("{line}");
                }
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
