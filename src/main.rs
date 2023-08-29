use anyhow::Result;
use clap::Parser;
use std::io::{stdout, BufWriter, Write};
use std::process::exit;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // The pattern
    string: Vec<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // "We store a list of integers in a vector called 'vec'. Then, we create an iterator using the 'iter' method. Next, we use the 'map' method to convert each integer to a string, and gather those strings into a new vector using the 'collect' method. Finally, we use the 'join' method to concatenate them into a string separated by spaces."
    let mut s = args
        .string
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");

    // "If no arguments are given, print 'y' repeatedly until killed."
    if s.len() == 0 {
        s = "y".to_string();
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
        // "If SIGINT is received, exit immediately with a status of 130."
        exit(130)
    })?;

    let out = stdout();
    let mut out = BufWriter::new(out.lock());
    while running.load(Ordering::SeqCst) {
        out.write(s.as_bytes())?;
        out.write(b"\n")?;
    }

    Ok(())
}
