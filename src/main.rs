use std::{process::exit, sync::mpsc, thread, time::Duration};

use anyhow::anyhow;
use clap::Parser;
use console::{Key, Term};
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
    loops: u64,
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
    term.hide_cursor()?;

    let mut inventory = Inventory::new();
    for path in args.path.split(";") {
        if let Err(e) = inventory.load_from_path(path) {
            eprintln!("Failed to load animation from {path}: {e}");
        }
    }

    if args.list {
        println!("Available animations:\n");

        for (name, animation) in inventory.animations() {
            println!("Name: {name}");
            animation.metadata.iter().for_each(|(k, v)| {
                println!("{k}: {v}");
            });
        }
        exit(0);
    }

    let Some(animation) = inventory.animation(&args.animation) else {
        return Err(anyhow!("Animation {} not found.", args.animation));
    };

    let n_frames = animation.frames.len() as u64;

    let (event_tx, event_rx) = mpsc::channel();
    let term_clone = term.clone();

    // No need to join
    let _event_listener = thread::spawn(move || {
        loop {
            let key = term_clone.read_key();
            if let Ok(k) = key {
                if k != Key::Unknown {
                    if event_tx.send(k).is_err() {
                        break;
                    }
                }
            }
        }
    });

    let timeout = Duration::from_millis(args.delay);
    for _ in 0..args.loops * n_frames {
        // TODO: draw

        let Ok(key) = event_rx.recv_timeout(timeout) else {
            eprintln!("Event listener closed.");
            break;
        };

        if key == Key::Escape || key == Key::CtrlC || key == Key::Char('q') {
            break;
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        exit(1);
    }
}
