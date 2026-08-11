use clap::Parser;
use rtodo::commands::{RTodoArgs, RTodoError};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), RTodoError> {
    let args = RTodoArgs::parse();
    rtodo::run(args)?;

    Ok(())
}
