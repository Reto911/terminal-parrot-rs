use std::{
    fmt::Display,
    process::exit,
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

use anyhow::anyhow;
use clap::{Parser, ValueEnum};
use console::{Key, Term};
use draw::Draw;
use inventory::Inventory;

mod animation;
mod colors;
mod draw;
mod inventory;

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq, Eq)]
enum Orientation {
    Regular,
    Aussie,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::Regular => write!(f, "regular"),
            Orientation::Aussie => write!(f, "aussie"),
        }
    }
}

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
    #[arg(long, default_value_t = Orientation::Regular)]
    orientation: Orientation,
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
            println!();
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
            if let Ok(k) = key
                && k != Key::Unknown
                && event_tx.send(k).is_err()
            {
                break;
            }
        }
    });

    let timeout = Duration::from_millis(args.delay);
    let mut draw = Draw::new(term, animation, args.orientation);

    let mut loop_idx = 0;
    loop {
        if args.loops > 0 && loop_idx >= args.loops * n_frames {
            break;
        }

        draw.draw()?;
        loop_idx += 1;
        match event_rx.recv_timeout(timeout) {
            Ok(key) => {
                if key == Key::Escape || key == Key::CtrlC || key == Key::Char('q') {
                    break;
                }
            }
            Err(RecvTimeoutError::Disconnected) => {
                eprintln!("Event listener closed.");
                break;
            }
            _ => {}
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
