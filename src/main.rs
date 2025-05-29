use std::process::exit;

use anyhow::anyhow;
use clap::Parser;
use console::Term;
use inventory::Inventory;

mod animation;
mod inventory;

#[derive(Debug, Parser)]
struct Args {
    /// path to additional frame files
    #[arg(default_value = "parrot")]
    animation: String,
    #[arg(long, default_value = "./animations")]
    path: String,
    /// number of times to loop (default: infinite)
    #[arg(long, default_value_t = 0)]
    loops: usize,
    /// frame delay in ms
    #[arg(long, default_value_t = 75)]
    delay: u64,
    /// regular or aussie
    #[arg(long, default_value = "regular")]
    orientation: String,
    /// list available animations and exit
    #[arg(long, short, default_value_t = false)]
    list: bool,
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let term = Term::stdout();
    if !term.is_term() {
        return Err(anyhow!("Should be run in a terminal."));
    }

    let mut inventory = Inventory::new();

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        exit(1);
    }
}
