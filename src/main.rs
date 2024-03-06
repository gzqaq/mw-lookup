use std::process;

use mw_lookup::{core_fn, Args};

use clap::Parser;

#[tokio::main]
async fn main() {
    core_fn(Args::parse()).await.unwrap_or_else(|err| {
        eprintln!("\x1b[1;31mUnable to look up!\x1b[0m\n{err}");
        process::exit(1);
    })
}
