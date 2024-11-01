use std::path::PathBuf;

use clap::Parser;
use pdf::file::FileOptions;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: PathBuf,

    /// Page number
    #[arg(short, long, default_value_t = 0)]
    page: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let old_file = FileOptions::cached().open(&args.input).unwrap();

    let pages: Vec<_> = old_file.pages().collect();

    dbg!(&pages[0]);

    Ok(())
}
