use anyhow::Result;
use clap::Parser;
use markdown::to_html;
use std::fs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// markdown to html.
struct Args {
    /// 입력 파일(들)
    #[arg(value_name = "FILE", required = true)]
    files: Vec<String>,
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match fs::read_to_string(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                let html = to_html(&file);
                println!("{html}")
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
