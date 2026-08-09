use std::error::Error;

use clap::Parser;
use rtodo::commands::RTodoArgs;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = RTodoArgs::parse();
    rtodo::run(args)?;

    Ok(())
}
